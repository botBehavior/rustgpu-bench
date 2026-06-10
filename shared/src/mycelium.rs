//! Mycelium — adaptive fungal ecosystem (T1: organism foundation).
//!
//! Hyphal TIPS grow a PERMANENT branching network. Each tip senses the nutrient
//! field ahead (chemotropism), steers up-gradient, advances, lays down permanent
//! `biomass`, forages (depletes nutrient), occasionally branches, and fuses
//! (anastomoses) when it runs into an established cord — bounding the population
//! and closing loops. T2 (this file) adds the resource-transport solver: a
//! biomass-weighted relaxation that ships sugar through the network, making tip
//! growth resource-limited. T3 adds the flux feedback that thickens cords. See
//! MYCELIA.md.
//!
//! Pure, CPU-testable, SPIR-V-subset clean (u32/f32, bounded, no recursion). Field
//! writes (biomass deposit, nutrient forage, tip append) are owned by the caller
//! so the math stays a pure function.

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use crate::rng;

const TAU: f32 = 2.0 * core::f32::consts::PI;

/// A hyphal tip. 6×f32 = 24 bytes, `repr(C)`. `alive`/`colony`/`age` are f32 to
/// keep the struct all-f32 (one layout everywhere); cast on use.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tip {
    pub x: f32,
    pub y: f32,
    pub heading: f32,
    pub colony: f32,
    pub alive: f32,
    pub age: f32,
}

impl Tip {
    pub const DEAD: Tip = Tip { x: 0.0, y: 0.0, heading: 0.0, colony: 0.0, alive: 0.0, age: 0.0 };
}

/// What one growth step produces; the caller applies the field writes (so the
/// math stays pure). Resource economy is applied at `home`; new biomass at `cell`.
pub struct Step {
    pub tip: Tip,            // the advanced (or stalled) tip; alive=0 if it anastomosed
    pub cell: u32,           // new cell — deposit biomass here when `advanced == 1`
    pub home: u32,           // current cell — subtract `forage`, add `resource_delta` here
    pub advanced: u32,       // 1 = grew/moved; 0 = resource-starved stall (held position)
    pub branched: u32,       // 1 if a child tip was spawned
    pub child: Tip,          // the child (only meaningful when branched == 1)
    pub forage: f32,         // nutrient absorbed at `home` (subtract from nutrient)
    pub resource_delta: f32, // resource change at `home` (income − cost); keeps R ≥ 0
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Params {
    pub width: u32,
    pub height: u32,
    pub n_tips: u32, // length of the tip array (max population)
    pub frame: u32,
    pub move_len: f32,
    pub sensor_angle: f32,
    pub sensor_dist: f32,
    pub turn: f32,
    pub deposit: f32,            // biomass laid per step
    pub forage_rate: f32,        // nutrient consumed per step at a cell
    pub branch_prob: f32,        // per-step branch probability
    pub branch_angle: f32,       // child heading offset
    pub min_branch_age: f32,     // steps before a tip may branch
    pub anastomosis_thresh: f32, // biomass-ahead that triggers fusion
    pub alpha: f32,              // transport relaxation rate (stable for alpha <= 0.25)
    pub k_half: f32,             // biomass at half conductivity (saturating k)
    pub growth_cost: f32,        // resource consumed to extend one step
    pub forage_income: f32,      // resource produced per unit nutrient foraged
    pub exposure: f32,           // render gain
    pub seed_tips: u32,          // initial live tips (germinating spore)
}

impl Params {
    pub fn default_for(width: u32, height: u32, n_tips: u32) -> Self {
        Params {
            width,
            height,
            n_tips,
            frame: 0,
            move_len: 1.0,
            sensor_angle: 0.5,
            sensor_dist: 6.0,
            turn: 0.35,
            deposit: 1.0,
            forage_rate: 0.2,
            branch_prob: 0.03,
            branch_angle: 0.6,
            min_branch_age: 6.0,
            anastomosis_thresh: 6.0,
            alpha: 0.2,
            k_half: 2.0,
            growth_cost: 0.5,
            forage_income: 1.0,
            exposure: 1.0,
            seed_tips: 12,
        }
    }
    #[inline]
    pub fn cells(&self) -> u32 {
        self.width * self.height
    }
}

fn wrap(v: f32, max: f32) -> f32 {
    if v < 0.0 {
        v + max
    } else if v >= max {
        v - max
    } else {
        v
    }
}

/// Cell index for a (wrapped) position.
pub fn index(x: f32, y: f32, p: &Params) -> u32 {
    let xi = (wrap(x, p.width as f32)) as u32 % p.width;
    let yi = (wrap(y, p.height as f32)) as u32 % p.height;
    yi * p.width + xi
}

/// Sample a field `dist` ahead of the tip, `angle_off` off its heading.
fn sense(tip: &Tip, angle_off: f32, field: &[f32], p: &Params) -> f32 {
    let a = tip.heading + angle_off;
    let sx = tip.x + a.cos() * p.sensor_dist;
    let sy = tip.y + a.sin() * p.sensor_dist;
    field[index(sx, sy, p) as usize]
}

/// Inoculate tip `i`: a germinating spore at center sends hyphae out radially.
/// First `seed` tips alive; the rest start dead and are filled by branching.
pub fn spawn_tip(i: u32, seed: u32, p: &Params) -> Tip {
    if i >= seed {
        return Tip::DEAD;
    }
    let h = (i as f32 / seed as f32) * TAU;
    Tip {
        x: p.width as f32 * 0.5,
        y: p.height as f32 * 0.5,
        heading: h,
        colony: 0.0,
        alive: 1.0,
        age: 0.0,
    }
}

/// One growth step for one (live) tip. Chemotropic steer toward nutrient, forage,
/// then — if the network can supply the build cost — advance, deposit, and decide
/// branch/anastomosis. Growth is **resource-limited**: a tip extends only when its
/// home cell holds enough transported resource (or it forages enough fresh sugar)
/// to pay `growth_cost`; otherwise it stalls and waits for the network to feed it.
/// Pure — caller applies the field writes named in `Step`.
pub fn grow_tip(
    tip: &Tip,
    nutrient: &[f32],
    biomass: &[f32],
    resource: &[f32],
    p: &Params,
    tip_id: u32,
) -> Step {
    let home = index(tip.x, tip.y, p);

    // chemotropism: steer toward the strongest nutrient ahead (Jones rule)
    let l = sense(tip, p.sensor_angle, nutrient, p);
    let c = sense(tip, 0.0, nutrient, p);
    let r = sense(tip, -p.sensor_angle, nutrient, p);
    let mut h = tip.heading;
    if c > l && c > r {
        // strongest ahead: hold
    } else if c < l && c < r {
        let mut s = rng::seed(tip_id, p.frame, 0x4F19);
        if rng::next_u32(&mut s) & 1 == 0 {
            h += p.turn;
        } else {
            h -= p.turn;
        }
    } else if l > r {
        h += p.turn;
    } else if r > l {
        h -= p.turn;
    }

    // forage at home: absorb nutrient -> resource (the network's income this step)
    let forage = nutrient[home as usize].min(p.forage_rate).max(0.0);
    let gain = forage * p.forage_income;

    // resource-limited growth gate: transported resource here plus fresh foraging
    // must cover the build cost. Tips sitting on food grow freely; tips at the
    // barren frontier grow only as fast as the network ships sugar to them.
    let supply = resource[home as usize] + gain;
    if tip.alive <= 0.0 || supply < p.growth_cost {
        // stall: still turn to face the gradient and forage, but hold position.
        let waited = Tip { heading: h, age: tip.age + 1.0, ..*tip };
        return Step {
            tip: waited,
            cell: home,
            home,
            advanced: 0,
            branched: 0,
            child: Tip::DEAD,
            forage,
            resource_delta: gain, // income only; no build cost paid
        };
    }

    // advance
    let nx = wrap(tip.x + h.cos() * p.move_len, p.width as f32);
    let ny = wrap(tip.y + h.sin() * p.move_len, p.height as f32);
    let cell = index(nx, ny, p);

    // anastomosis: if we grew into an established cord, fuse (deposit, then die)
    let mut alive = 1.0;
    if tip.age > p.min_branch_age && biomass[cell as usize] > p.anastomosis_thresh {
        alive = 0.0;
    }

    let advanced = Tip {
        x: nx,
        y: ny,
        heading: h,
        colony: tip.colony,
        alive,
        age: tip.age + 1.0,
    };

    // branch: deterministic from (tip_id, frame); a child peels off at an angle
    let mut branched = 0u32;
    let mut child = Tip::DEAD;
    if alive > 0.0 && tip.age > p.min_branch_age {
        let mut s = rng::seed(tip_id, p.frame, 0xB3A7);
        if rng::next_f32(&mut s) < p.branch_prob {
            branched = 1;
            let side = if rng::next_u32(&mut s) & 1 == 0 { 1.0 } else { -1.0 };
            child = Tip {
                x: nx,
                y: ny,
                heading: h + side * p.branch_angle,
                colony: tip.colony,
                alive: 1.0,
                age: 0.0,
            };
        }
    }

    Step {
        tip: advanced,
        cell,
        home,
        advanced: 1,
        branched,
        child,
        forage,
        resource_delta: gain - p.growth_cost, // income minus build cost; R stays ≥ 0
    }
}

/// Resource flow through one cell after a transport iteration.
pub struct Flow {
    pub resource: f32, // R'[i] after one Jacobi relaxation
    pub flux: f32,     // |throughput| through this cell this iter (drives T3 adaptation)
}

/// Saturating conductivity between two biomass densities: ~0 in open space,
/// →1 along thick cords. Saturating (rather than raw `min(B_i,B_j)`) keeps the
/// solver stable (α ≤ ¼) no matter how thick cords grow in T3, and is itself
/// biological (saturating uptake). `m = min(B_i,B_j)` so it stays symmetric —
/// the property that makes the relaxation mass-conserving.
#[inline]
pub fn conductivity(bi: f32, bj: f32, k_half: f32) -> f32 {
    let m = bi.min(bj);
    m / (m + k_half)
}

/// One Jacobi relaxation of the transport solver at cell (x, y): resource diffuses
/// along biomass and we accumulate throughput magnitude. Toroidal 4-neighbour, to
/// match `index()`'s wrap. Because conductivity is symmetric the sweep is
/// **mass-conserving** (no source/sink), and for α ≤ ¼ it obeys the maximum
/// principle, so resource stays in `[0, max]` with no clamp. Run K of these per
/// frame (caller ping-pongs the resource buffer) so sugar propagates several cells.
pub fn transport_at(x: u32, y: u32, resource: &[f32], biomass: &[f32], p: &Params) -> Flow {
    let w = p.width;
    let h = p.height;
    let i = (y * w + x) as usize;
    let ri = resource[i];
    let bi = biomass[i];

    let xm = if x == 0 { w - 1 } else { x - 1 };
    let xp = if x + 1 == w { 0 } else { x + 1 };
    let ym = if y == 0 { h - 1 } else { y - 1 };
    let yp = if y + 1 == h { 0 } else { y + 1 };
    let nbr = [
        (ym * w + x) as usize,
        (yp * w + x) as usize,
        (y * w + xm) as usize,
        (y * w + xp) as usize,
    ];

    let mut dr = 0.0f32;
    let mut flux = 0.0f32;
    let mut n = 0;
    while n < 4 {
        let j = nbr[n];
        let k = conductivity(bi, biomass[j], p.k_half);
        let diff = resource[j] - ri;
        dr += k * diff;
        flux += k * diff.abs();
        n += 1;
    }
    Flow { resource: ri + p.alpha * dr, flux }
}

/// Render math (tested Rust): biomass -> bioluminescent foxfire glow, remaining
/// nutrient -> faint warm substrate. Dark where neither.
pub fn shade(biomass: f32, nutrient: f32, exposure: f32) -> glam::Vec3 {
    use glam::vec3;
    let sub = vec3(0.05, 0.035, 0.02) * (1.0 - (-(nutrient * 2.0).max(0.0)).exp());
    let g = 1.0 - (-(biomass * exposure).max(0.0)).exp();
    // cyan-green glow with a brighter, whiter core where biomass is thick
    let glow = vec3(0.20, 1.0, 0.70) * g + vec3(0.5, 0.95, 1.0) * (g * g) * 0.6;
    let lin = sub + glow;
    let tm = |v: f32| {
        let m = v / (1.0 + v);
        m.max(0.0).powf(0.4545)
    };
    vec3(tm(lin.x), tm(lin.y), tm(lin.z))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params() -> Params {
        Params::default_for(64, 48, 256)
    }

    #[test]
    fn spawn_radial_then_dead() {
        let p = params();
        let seed = 8;
        let cx = p.width as f32 * 0.5;
        let cy = p.height as f32 * 0.5;
        for i in 0..seed {
            let t = spawn_tip(i, seed, &p);
            assert_eq!(t, spawn_tip(i, seed, &p));
            assert!(t.alive > 0.0);
            assert_eq!((t.x, t.y), (cx, cy));
        }
        // distinct headings, and the rest of the array is dead
        assert_ne!(spawn_tip(0, seed, &p).heading, spawn_tip(1, seed, &p).heading);
        assert_eq!(spawn_tip(seed, seed, &p).alive, 0.0);
    }

    /// Resource everywhere so growth isn't gated — for testing the T1 behaviours.
    fn flush(p: &Params) -> Vec<f32> {
        vec![10.0f32; p.cells() as usize]
    }

    #[test]
    fn steers_toward_nutrient() {
        let p = params();
        let resource = flush(&p);
        let nutrient_zero = vec![0.0f32; p.cells() as usize];
        let mut nutrient = vec![0.0f32; p.cells() as usize];
        let biomass = vec![0.0f32; p.cells() as usize];
        let tip = Tip { x: 32.0, y: 24.0, heading: 0.0, colony: 0.0, alive: 1.0, age: 0.0 };
        // pile nutrient at the LEFT sensor location
        let sx = tip.x + (tip.heading + p.sensor_angle).cos() * p.sensor_dist;
        let sy = tip.y + (tip.heading + p.sensor_angle).sin() * p.sensor_dist;
        nutrient[index(sx, sy, &p) as usize] = 5.0;
        let step = grow_tip(&tip, &nutrient, &biomass, &resource, &p, 0);
        assert!(step.tip.heading > tip.heading, "should turn toward food");
        // with no nutrient anywhere, it holds course
        let flat = grow_tip(&tip, &nutrient_zero, &biomass, &resource, &p, 0);
        assert_eq!(flat.tip.heading, tip.heading);
    }

    #[test]
    fn advances_and_deposits_at_new_cell() {
        let p = params();
        let resource = flush(&p);
        let nutrient = vec![0.0f32; p.cells() as usize];
        let biomass = vec![0.0f32; p.cells() as usize];
        let tip = Tip { x: 10.0, y: 10.0, heading: 0.0, colony: 0.0, alive: 1.0, age: 1.0 };
        let step = grow_tip(&tip, &nutrient, &biomass, &resource, &p, 0);
        assert_eq!(step.advanced, 1);
        assert!((step.tip.x - 11.0).abs() < 1e-4, "moved +x by move_len");
        assert_eq!(step.cell, index(step.tip.x, step.tip.y, &p));
        assert_eq!(step.home, index(10.0, 10.0, &p));
        assert_eq!(step.tip.age, 2.0);
    }

    #[test]
    fn branches_when_forced() {
        let mut p = params();
        p.branch_prob = 1.0; // force a branch
        let resource = flush(&p);
        let nutrient = vec![0.0f32; p.cells() as usize];
        let biomass = vec![0.0f32; p.cells() as usize];
        let tip = Tip { x: 20.0, y: 20.0, heading: 0.7, colony: 0.0, alive: 1.0, age: 10.0 };
        let step = grow_tip(&tip, &nutrient, &biomass, &resource, &p, 7);
        assert_eq!(step.branched, 1);
        assert!(step.child.alive > 0.0 && step.child.age == 0.0);
        assert!((step.child.heading - step.tip.heading).abs() > 1e-3, "child peels off at an angle");
        // a freshly-spawned tip (age 0) below min_branch_age does NOT branch
        let young = Tip { age: 0.0, ..tip };
        assert_eq!(grow_tip(&young, &nutrient, &biomass, &resource, &p, 7).branched, 0);
    }

    #[test]
    fn anastomoses_into_established_cord() {
        let p = params();
        let resource = flush(&p);
        let nutrient = vec![0.0f32; p.cells() as usize];
        let mut biomass = vec![0.0f32; p.cells() as usize];
        let tip = Tip { x: 30.0, y: 30.0, heading: 0.0, colony: 0.0, alive: 1.0, age: 20.0 };
        // lay a thick cord at the cell the tip will step into
        let nx = wrap(tip.x + p.move_len, p.width as f32);
        biomass[index(nx, tip.y, &p) as usize] = p.anastomosis_thresh + 1.0;
        let step = grow_tip(&tip, &nutrient, &biomass, &resource, &p, 0);
        assert_eq!(step.tip.alive, 0.0, "tip fuses into the network and stops");
    }

    // ---- T2: the transport solver ------------------------------------------

    #[test]
    fn transport_conserves_resource_with_no_source_or_sink() {
        let p = Params::default_for(8, 8, 0);
        let n = p.cells() as usize;
        let mut biomass = vec![0.0f32; n];
        let mut resource = vec![0.0f32; n];
        for y in 2..6 {
            for x in 2..6 {
                biomass[(y * 8 + x) as usize] = 3.0;
            }
        }
        resource[3 * 8 + 3] = 10.0;
        resource[4 * 8 + 4] = 4.0;
        let before: f32 = resource.iter().sum();
        let max_before = resource.iter().cloned().fold(0.0f32, f32::max);

        // one Jacobi sweep (read old, write new)
        let mut next = resource.clone();
        for y in 0..8 {
            for x in 0..8 {
                next[(y * 8 + x) as usize] = transport_at(x, y, &resource, &biomass, &p).resource;
            }
        }
        let after: f32 = next.iter().sum();
        assert!((after - before).abs() < 1e-3, "resource conserved: {before} -> {after}");
        // maximum principle (α ≤ ¼): nothing goes negative or above the prior max
        for &v in &next {
            assert!(v >= -1e-6 && v <= max_before + 1e-6, "out of bounds: {v}");
        }
    }

    #[test]
    fn resource_flows_along_biomass_not_through_gaps() {
        let p = Params::default_for(16, 3, 0);
        let mut biomass = vec![0.0f32; p.cells() as usize];
        let mut resource = vec![0.0f32; p.cells() as usize];
        // a horizontal cord on row 1, x = 1..=8; source pumped in at the near end
        let row = 1u32;
        for x in 1..=8 {
            biomass[(row * 16 + x) as usize] = 5.0;
        }
        let src = (row * 16 + 1) as usize;
        let sink = (row * 16 + 8) as usize;
        resource[src] = 20.0;
        // an isolated high-R cell with NO biomass must not bleed out
        let island = (row * 16 + 13) as usize;
        resource[island] = 20.0;

        for _ in 0..200 {
            let mut next = resource.clone();
            for y in 0..p.height {
                for x in 0..p.width {
                    next[(y * p.width + x) as usize] =
                        transport_at(x, y, &resource, &biomass, &p).resource;
                }
            }
            resource = next;
        }
        assert!(resource[sink] > 0.5, "resource reached the far end of the cord: {}", resource[sink]);
        assert!(resource[src] < 20.0, "source drained as it fed the cord: {}", resource[src]);
        assert!(
            (resource[island] - 20.0).abs() < 1e-3,
            "an isolated cell with no biomass cannot transport: {}",
            resource[island]
        );
    }

    #[test]
    fn no_biomass_means_frozen_resource() {
        let p = Params::default_for(8, 8, 0);
        let biomass = vec![0.0f32; p.cells() as usize];
        let mut resource = vec![0.0f32; p.cells() as usize];
        resource[2 * 8 + 4] = 7.0;
        let f = transport_at(4, 2, &resource, &biomass, &p);
        assert_eq!(f.resource, 7.0, "no conductivity => resource cannot move");
        assert_eq!(f.flux, 0.0);
    }

    #[test]
    fn flux_tracks_gradient_along_cord() {
        let p = Params::default_for(8, 8, 0);
        let mut biomass = vec![0.0f32; p.cells() as usize];
        for x in 0..8 {
            biomass[(3 * 8 + x) as usize] = 5.0; // a cord across row 3
        }
        // uniform resource on the cord => no gradient => zero flux
        let mut uni = vec![0.0f32; p.cells() as usize];
        for x in 0..8 {
            uni[(3 * 8 + x) as usize] = 4.0;
        }
        assert_eq!(transport_at(4, 3, &uni, &biomass, &p).flux, 0.0);
        // a gradient along the cord => positive flux
        let mut grad = vec![0.0f32; p.cells() as usize];
        for x in 0..8 {
            grad[(3 * 8 + x) as usize] = x as f32;
        }
        assert!(transport_at(4, 3, &grad, &biomass, &p).flux > 0.0);
    }

    #[test]
    fn growth_is_resource_limited() {
        let p = params();
        let n = p.cells() as usize;
        let nutrient = vec![0.0f32; n];
        let biomass = vec![0.0f32; n];
        let tip = Tip { x: 20.0, y: 20.0, heading: 0.0, colony: 0.0, alive: 1.0, age: 3.0 };

        // barren + no resource => stall: no advance, position held, no branch
        let dry = vec![0.0f32; n];
        let s = grow_tip(&tip, &nutrient, &biomass, &dry, &p, 0);
        assert_eq!(s.advanced, 0, "a starved tip stalls");
        assert_eq!((s.tip.x, s.tip.y), (tip.x, tip.y), "stalled tip holds position");
        assert_eq!(s.branched, 0);
        assert_eq!(s.resource_delta, 0.0, "nothing to forage, nothing spent");

        // transported resource at home => advances and pays exactly growth_cost
        let mut rich = vec![0.0f32; n];
        rich[index(tip.x, tip.y, &p) as usize] = 10.0;
        let g = grow_tip(&tip, &nutrient, &biomass, &rich, &p, 0);
        assert_eq!(g.advanced, 1, "transported resource lets it grow");
        assert!((g.resource_delta + p.growth_cost).abs() < 1e-6, "paid growth_cost, no forage income");
    }

    #[test]
    fn foraging_on_food_funds_growth_and_depletes_nutrient() {
        let mut p = params();
        p.forage_income = 5.0; // rich food
        p.growth_cost = 0.5;
        let n = p.cells() as usize;
        let dry = vec![0.0f32; n];
        let biomass = vec![0.0f32; n];
        let mut nutrient = vec![0.0f32; n];
        let tip = Tip { x: 20.0, y: 20.0, heading: 0.0, colony: 0.0, alive: 1.0, age: 3.0 };
        let home = index(tip.x, tip.y, &p) as usize;
        nutrient[home] = 1.0; // food right here, zero transported resource

        let s = grow_tip(&tip, &nutrient, &biomass, &dry, &p, 0);
        assert_eq!(s.advanced, 1, "a tip on food makes its own sugar and grows");
        assert_eq!(s.home, home as u32);
        assert!(s.forage > 0.0 && s.forage <= p.forage_rate, "consumed nutrient at home");
        // resource_delta == forage*income − growth_cost, and a rich find leaves surplus
        let expected = s.forage * p.forage_income - p.growth_cost;
        assert!((s.resource_delta - expected).abs() < 1e-5);
        assert!(s.resource_delta > 0.0, "rich food leaves surplus resource to transport");
    }

    #[test]
    fn shade_bounded_and_dark_when_empty() {
        for &(b, n) in &[(0.0, 0.0), (1.0, 0.0), (10.0, 3.0), (0.0, 5.0)] {
            let col = shade(b, n, 1.0);
            for v in [col.x, col.y, col.z] {
                assert!(v.is_finite() && (0.0..=1.0).contains(&v));
            }
        }
        assert_eq!(shade(0.0, 0.0, 1.0), glam::Vec3::ZERO);
        assert!(shade(4.0, 0.0, 1.0).y > shade(0.4, 0.0, 1.0).y, "denser mycelium glows brighter");
    }
}
