//! Mycelium — adaptive fungal ecosystem (T1: organism foundation).
//!
//! Hyphal TIPS grow a PERMANENT branching network. Each tip senses the nutrient
//! field ahead (chemotropism), steers up-gradient, advances, lays down permanent
//! `biomass`, forages (depletes nutrient), occasionally branches, and fuses
//! (anastomoses) when it runs into an established cord — bounding the population
//! and closing loops. T2 adds the resource-transport solver that makes growth
//! resource-limited; T3 adds the flux feedback that thickens cords. See MYCELIA.md.
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

/// What one growth step produces; the caller applies the field writes.
pub struct Step {
    pub tip: Tip,      // the advanced tip (alive=0 if it anastomosed/terminated)
    pub cell: u32,     // deposit biomass AND forage nutrient here
    pub branched: u32, // 1 if a child tip was spawned
    pub child: Tip,    // the child (only meaningful when branched==1)
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

/// One growth step for one (live) tip. Chemotropic steer toward nutrient, advance,
/// decide branch/anastomosis. Pure — caller applies `cell` (deposit+forage) and
/// appends `child` when `branched`.
pub fn grow_tip(tip: &Tip, nutrient: &[f32], biomass: &[f32], p: &Params, tip_id: u32) -> Step {
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

    // advance
    let nx = wrap(tip.x + h.cos() * p.move_len, p.width as f32);
    let ny = wrap(tip.y + h.sin() * p.move_len, p.height as f32);
    let cell = index(nx, ny, p);

    // anastomosis: if we walked into an established cord, fuse (deposit, then die)
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

    Step { tip: advanced, cell, branched, child }
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

    #[test]
    fn steers_toward_nutrient() {
        let p = params();
        let nutrient_zero = vec![0.0f32; p.cells() as usize];
        let mut nutrient = vec![0.0f32; p.cells() as usize];
        let biomass = vec![0.0f32; p.cells() as usize];
        let tip = Tip { x: 32.0, y: 24.0, heading: 0.0, colony: 0.0, alive: 1.0, age: 0.0 };
        // pile nutrient at the LEFT sensor location
        let sx = tip.x + (tip.heading + p.sensor_angle).cos() * p.sensor_dist;
        let sy = tip.y + (tip.heading + p.sensor_angle).sin() * p.sensor_dist;
        nutrient[index(sx, sy, &p) as usize] = 5.0;
        let step = grow_tip(&tip, &nutrient, &biomass, &p, 0);
        assert!(step.tip.heading > tip.heading, "should turn toward food");
        // with no nutrient anywhere, it holds course
        let flat = grow_tip(&tip, &nutrient_zero, &biomass, &p, 0);
        assert_eq!(flat.tip.heading, tip.heading);
    }

    #[test]
    fn advances_and_deposits_at_new_cell() {
        let p = params();
        let nutrient = vec![0.0f32; p.cells() as usize];
        let biomass = vec![0.0f32; p.cells() as usize];
        let tip = Tip { x: 10.0, y: 10.0, heading: 0.0, colony: 0.0, alive: 1.0, age: 1.0 };
        let step = grow_tip(&tip, &nutrient, &biomass, &p, 0);
        assert!((step.tip.x - 11.0).abs() < 1e-4, "moved +x by move_len");
        assert_eq!(step.cell, index(step.tip.x, step.tip.y, &p));
        assert_eq!(step.tip.age, 2.0);
    }

    #[test]
    fn branches_when_forced() {
        let mut p = params();
        p.branch_prob = 1.0; // force a branch
        let nutrient = vec![0.0f32; p.cells() as usize];
        let biomass = vec![0.0f32; p.cells() as usize];
        let tip = Tip { x: 20.0, y: 20.0, heading: 0.7, colony: 0.0, alive: 1.0, age: 10.0 };
        let step = grow_tip(&tip, &nutrient, &biomass, &p, 7);
        assert_eq!(step.branched, 1);
        assert!(step.child.alive > 0.0 && step.child.age == 0.0);
        assert!((step.child.heading - step.tip.heading).abs() > 1e-3, "child peels off at an angle");
        // a freshly-spawned tip (age 0) below min_branch_age does NOT branch
        let young = Tip { age: 0.0, ..tip };
        assert_eq!(grow_tip(&young, &nutrient, &biomass, &p, 7).branched, 0);
    }

    #[test]
    fn anastomoses_into_established_cord() {
        let p = params();
        let nutrient = vec![0.0f32; p.cells() as usize];
        let mut biomass = vec![0.0f32; p.cells() as usize];
        let tip = Tip { x: 30.0, y: 30.0, heading: 0.0, colony: 0.0, alive: 1.0, age: 20.0 };
        // lay a thick cord at the cell the tip will step into
        let nx = wrap(tip.x + p.move_len, p.width as f32);
        biomass[index(nx, tip.y, &p) as usize] = p.anastomosis_thresh + 1.0;
        let step = grow_tip(&tip, &nutrient, &biomass, &p, 0);
        assert_eq!(step.tip.alive, 0.0, "tip fuses into the network and stops");
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
