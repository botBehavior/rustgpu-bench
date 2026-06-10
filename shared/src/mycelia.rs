//! Mycelia — multi-species Physarum, the flagship visual spectacle kernel.
//!
//! Several agent populations ("species") share the field. Each senses its OWN
//! trail channel (attraction) and is REPELLED by the others, so the colonies
//! carve out competing territories with luminous boundaries — far richer than
//! single-species slime. Mouse force is part of the kernel so the field is
//! genuinely interactive.
//!
//! Everything here is ordinary Rust with CPU unit tests, compiled UNCHANGED to
//! SPIR-V by rust-gpu. That is the whole point: a million-agent living sim whose
//! logic you can read, test on the CPU, and run on the GPU. u32/f32 only, no
//! usize in data, bounded loops — SPIR-V-subset clean.
//!
//! Trail layout is PLANAR: channel `c`, cell `i` -> `trail[c * w*h + i]`.
//! Deposits are non-atomic (races lose a few, visually irrelevant at scale);
//! single-agent runs and the diffuse pass stay deterministic for GPU-vs-CPU
//! verification.

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use crate::rng;

/// Number of competing species / trail channels. Rendered to R, G, B.
pub const N_SPECIES: u32 = 3;

const TAU: f32 = 2.0 * core::f32::consts::PI;

/// One agent: position (px), heading (rad), species index in `0..N_SPECIES`.
/// 16 bytes, `repr(C)` — identical on CPU and GPU.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Agent {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    /// species index, stored as f32 to keep the struct all-f32 (cast on use)
    pub species: f32,
}

/// Simulation + interaction parameters. All u32/f32, flat `repr(C)`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Params {
    pub width: u32,
    pub height: u32,
    pub n_agents: u32,
    pub frame: u32,
    pub move_speed: f32,
    pub turn_speed: f32,
    pub sensor_angle: f32, // radians off heading
    pub sensor_dist: f32,  // px ahead
    pub deposit: f32,      // trail added per agent per step
    pub decay: f32,        // trail multiplier per diffuse step
    pub self_attract: f32, // pull toward own channel
    pub other_repel: f32,  // push away from other channels
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_force: f32, // >0 attract toward cursor, <0 repel, 0 off
    pub mouse_radius: f32,
    pub exposure: f32, // render gain
}

impl Params {
    pub fn default_for(width: u32, height: u32, n_agents: u32) -> Self {
        Params {
            width,
            height,
            n_agents,
            frame: 0,
            move_speed: 1.0,
            turn_speed: 0.38,
            sensor_angle: 0.5,
            sensor_dist: 9.0,
            deposit: 1.0,
            decay: 0.96,
            self_attract: 1.0,
            other_repel: 1.0,
            mouse_x: -1.0,
            mouse_y: -1.0,
            mouse_force: 0.0,
            mouse_radius: 120.0,
            exposure: 1.0,
        }
    }
    #[inline]
    pub fn field_cells(&self) -> u32 {
        self.width * self.height
    }
}

/// Toroidal wrap into [0, max).
fn wrap(v: f32, max: f32) -> f32 {
    if v < 0.0 {
        v + max
    } else if v >= max {
        v - max
    } else {
        v
    }
}

/// Planar trail index for channel `c` at a (wrapped) position.
pub fn trail_index(c: u32, x: f32, y: f32, p: &Params) -> u32 {
    let xi = (wrap(x, p.width as f32)) as u32 % p.width;
    let yi = (wrap(y, p.height as f32)) as u32 % p.height;
    c * p.field_cells() + yi * p.width + xi
}

/// Weighted signal at a sensor point: own channel pulls, others push.
fn sense(agent: &Agent, angle_off: f32, trail: &[f32], p: &Params) -> f32 {
    let a = agent.angle + angle_off;
    let sx = agent.x + a.cos() * p.sensor_dist;
    let sy = agent.y + a.sin() * p.sensor_dist;
    let me = agent.species as u32;
    let mut signal = 0.0f32;
    let mut c = 0u32;
    while c < N_SPECIES {
        let v = trail[trail_index(c, sx, sy, p) as usize];
        if c == me {
            signal += p.self_attract * v;
        } else {
            signal -= p.other_repel * v;
        }
        c += 1;
    }
    signal
}

/// Spawn agent `i`: scattered across the field, random heading, species by
/// thirds. Deterministic from the index.
pub fn spawn_agent(i: u32, p: &Params) -> Agent {
    let mut state = rng::seed(i, 0xC0FFEE, p.frame);
    let x = rng::next_f32(&mut state) * p.width as f32;
    let y = rng::next_f32(&mut state) * p.height as f32;
    let angle = rng::next_f32(&mut state) * TAU;
    let species = (i % N_SPECIES) as f32;
    Agent { x, y, angle, species }
}

/// One step for one agent: sense L/C/R, steer, apply mouse force, move, wrap.
/// Returns the new agent and the deposit index in ITS OWN channel.
pub fn update_agent(agent: &Agent, trail: &[f32], p: &Params, agent_index: u32) -> (Agent, u32) {
    let left = sense(agent, p.sensor_angle, trail, p);
    let center = sense(agent, 0.0, trail, p);
    let right = sense(agent, -p.sensor_angle, trail, p);

    // canonical Jeff Jones agent rule, generalized to signed (repulsive) signals
    let mut angle = agent.angle;
    if center > left && center > right {
        // strongest signal straight ahead: hold course
    } else if center < left && center < right {
        // center is strictly weakest: turn a random way (deterministic)
        let mut state = rng::seed(agent_index, p.frame, 0x51AE);
        if rng::next_u32(&mut state) & 1 == 0 {
            angle += p.turn_speed;
        } else {
            angle -= p.turn_speed;
        }
    } else if left > right {
        angle += p.turn_speed; // steer toward the stronger (left) sensor
    } else if right > left {
        angle -= p.turn_speed; // steer toward the stronger (right) sensor
    }

    // mouse force: steer toward/away from the cursor, falling off with distance
    if p.mouse_force != 0.0 && p.mouse_x >= 0.0 {
        let dx = p.mouse_x - agent.x;
        let dy = p.mouse_y - agent.y;
        let d2 = dx * dx + dy * dy;
        let r2 = p.mouse_radius * p.mouse_radius;
        if d2 < r2 && d2 > 0.0001 {
            let target = dy.atan2(dx);
            // blend heading toward (force>0) or away (force<0) from the target
            let aim = if p.mouse_force < 0.0 { target + core::f32::consts::PI } else { target };
            let falloff = 1.0 - d2 / r2;
            let mut diff = aim - angle;
            // shortest angular direction
            while diff > core::f32::consts::PI {
                diff -= TAU;
            }
            while diff < -core::f32::consts::PI {
                diff += TAU;
            }
            angle += diff * p.mouse_force.abs() * falloff * 0.5;
        }
    }

    let x = wrap(agent.x + angle.cos() * p.move_speed, p.width as f32);
    let y = wrap(agent.y + angle.sin() * p.move_speed, p.height as f32);
    let next = Agent { x, y, angle, species: agent.species };
    let cell = trail_index(agent.species as u32, x, y, p);
    (next, cell)
}

/// Diffuse + decay one cell of one channel: 3×3 toroidal mean × decay.
pub fn diffuse_at(trail: &[f32], c: u32, x: u32, y: u32, p: &Params) -> f32 {
    let w = p.width;
    let h = p.height;
    let base = c * p.field_cells();
    let mut sum = 0.0f32;
    let mut dy = 0u32;
    while dy < 3 {
        let mut dx = 0u32;
        while dx < 3 {
            let nx = (x + w + dx - 1) % w;
            let ny = (y + h + dy - 1) % h;
            sum += trail[(base + ny * w + nx) as usize];
            dx += 1;
        }
        dy += 1;
    }
    (sum / 9.0) * p.decay
}

/// Render math — kept in tested Rust on purpose. Maps the three trail-channel
/// intensities at a pixel to a luminous linear-then-tonemapped color. Each
/// species owns a base hue (warm / cool / acid); overlaps blend, so the
/// territory boundaries glow with mixed colors. `exposure` scales brightness.
pub fn shade(c0: f32, c1: f32, c2: f32, exposure: f32) -> glam::Vec3 {
    use glam::vec3;
    // saturating brightness per channel: dense trail -> ~1, empty -> 0
    let b = |c: f32| 1.0 - (-(c * exposure).max(0.0)).exp();
    let col0 = vec3(1.0, 0.42, 0.12); // ember
    let col1 = vec3(0.14, 0.72, 1.0); // ice
    let col2 = vec3(0.62, 1.0, 0.24); // acid
    let lin = col0 * b(c0) + col1 * b(c1) + col2 * b(c2);
    // Reinhard tonemap + approximate sRGB gamma, clamped to [0,1]
    let tm = |x: f32| {
        let m = x / (1.0 + x);
        m.max(0.0).powf(0.4545)
    };
    vec3(tm(lin.x), tm(lin.y), tm(lin.z))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params() -> Params {
        Params::default_for(64, 48, 9)
    }

    #[test]
    fn spawn_deterministic_in_bounds_all_species() {
        let p = params();
        let mut seen = [false; N_SPECIES as usize];
        for i in 0..9 {
            let a = spawn_agent(i, &p);
            assert_eq!(a, spawn_agent(i, &p));
            assert!(a.x >= 0.0 && a.x < p.width as f32);
            assert!(a.y >= 0.0 && a.y < p.height as f32);
            assert!((a.species as u32) < N_SPECIES);
            seen[a.species as usize] = true;
        }
        assert!(seen.iter().all(|&s| s), "all species spawned");
    }

    #[test]
    fn deposit_targets_own_channel() {
        let p = params();
        let trail = vec![0.0f32; (N_SPECIES * p.field_cells()) as usize];
        let agent = Agent { x: 20.0, y: 20.0, angle: 0.3, species: 2.0 };
        let (_next, cell) = update_agent(&agent, &trail, &p, 0);
        // cell must land in channel 2's slab
        assert!(cell >= 2 * p.field_cells() && cell < 3 * p.field_cells());
    }

    #[test]
    fn repelled_by_other_species_trail() {
        // species 0 agent with species-1 trail piled at its LEFT sensor should
        // steer RIGHT (away), the opposite of attraction.
        let mut p = params();
        p.self_attract = 1.0;
        p.other_repel = 1.0;
        let trail_cells = (N_SPECIES * p.field_cells()) as usize;
        let mut trail = vec![0.0f32; trail_cells];
        let agent = Agent { x: 32.0, y: 24.0, angle: 0.0, species: 0.0 };
        let sx = agent.x + (agent.angle + p.sensor_angle).cos() * p.sensor_dist;
        let sy = agent.y + (agent.angle + p.sensor_angle).sin() * p.sensor_dist;
        // pile SPECIES 1 trail at the left sensor
        trail[trail_index(1, sx, sy, &p) as usize] = 10.0;
        let (next, _) = update_agent(&agent, &trail, &p, 0);
        assert!(next.angle < agent.angle, "should turn away from rival trail");
    }

    #[test]
    fn attracted_to_own_species_trail() {
        let p = params();
        let mut trail = vec![0.0f32; (N_SPECIES * p.field_cells()) as usize];
        let agent = Agent { x: 32.0, y: 24.0, angle: 0.0, species: 0.0 };
        let sx = agent.x + (agent.angle + p.sensor_angle).cos() * p.sensor_dist;
        let sy = agent.y + (agent.angle + p.sensor_angle).sin() * p.sensor_dist;
        trail[trail_index(0, sx, sy, &p) as usize] = 10.0;
        let (next, _) = update_agent(&agent, &trail, &p, 0);
        assert!(next.angle > agent.angle, "should turn toward own trail");
    }

    #[test]
    fn mouse_attracts() {
        let mut p = params();
        p.mouse_force = 1.0;
        p.mouse_x = 50.0;
        p.mouse_y = 24.0;
        p.mouse_radius = 100.0;
        // agent at (20,24) heading away (angle = PI, facing -x); mouse is +x.
        let agent = Agent { x: 20.0, y: 24.0, angle: core::f32::consts::PI, species: 0.0 };
        let trail = vec![0.0f32; (N_SPECIES * p.field_cells()) as usize];
        let (next, _) = update_agent(&agent, &trail, &p, 0);
        // heading should rotate toward the cursor (target angle 0)
        let toward = (next.angle.cos()) > agent.angle.cos();
        assert!(toward, "mouse attraction should bend heading toward +x");
    }

    #[test]
    fn shade_is_bounded_and_monotonic() {
        for &(a, b, c) in &[(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (5.0, 5.0, 5.0), (100.0, 0.0, 0.0)] {
            let col = shade(a, b, c, 1.0);
            for v in [col.x, col.y, col.z] {
                assert!(v.is_finite() && (0.0..=1.0).contains(&v), "{v} out of range");
            }
        }
        // more trail in a channel => brighter in that channel's dominant hue
        let dim = shade(0.2, 0.0, 0.0, 1.0);
        let bright = shade(4.0, 0.0, 0.0, 1.0);
        assert!(bright.x > dim.x, "denser ember trail should be brighter");
        // empty field renders black
        assert_eq!(shade(0.0, 0.0, 0.0, 1.0), glam::Vec3::ZERO);
    }

    #[test]
    fn diffuse_decays_mass_per_channel() {
        let p = params();
        let cells = p.field_cells();
        let mut trail = vec![0.0f32; (N_SPECIES * cells) as usize];
        // spike in channel 1 only
        trail[(1 * cells + 24 * p.width + 32) as usize] = 9.0;
        let next: Vec<f32> = (0..cells)
            .map(|i| diffuse_at(&trail, 1, i % p.width, i / p.width, &p))
            .collect();
        let after: f32 = next.iter().sum();
        assert!((after - 9.0 * p.decay).abs() < 1e-3);
        assert_eq!(next.iter().filter(|v| **v > 0.0).count(), 9);
        // channels 0 and 2 untouched
        let ch0: f32 = (0..cells).map(|i| diffuse_at(&trail, 0, i % p.width, i / p.width, &p)).sum();
        assert_eq!(ch0, 0.0);
    }
}
