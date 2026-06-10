//! Physarum (slime mold) simulation kernels — pure functions, CPU-testable,
//! compiled unchanged to SPIR-V. u32/f32 only; no usize in data; bounded loops.
//!
//! Per frame: every agent senses the trail at three points ahead (left/center/
//! right), steers toward the strongest signal, moves, and deposits trail; then
//! the trail field diffuses (3×3 mean) and decays. Emergent transport networks.
//!
//! Determinism note (honest): agents deposit non-atomically in v1 — concurrent
//! GPU agents racing the same cell lose deposits occasionally, which is
//! visually irrelevant but means multi-agent GPU runs aren't bit-comparable to
//! CPU. Single-agent runs and the diffuse kernel ARE deterministic and are
//! what GPU-vs-CPU verification gates on.

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use crate::rng;

/// One agent: position in pixels, heading in radians. 16 bytes, `repr(C)`.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Agent {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub _pad: f32,
}

/// Simulation parameters. u32/f32 only — identical layout everywhere.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SimParams {
    pub width: u32,
    pub height: u32,
    pub n_agents: u32,
    pub frame: u32,
    pub move_speed: f32,   // px per step
    pub turn_speed: f32,   // radians per step
    pub sensor_angle: f32, // radians off heading
    pub sensor_dist: f32,  // px ahead
    pub deposit: f32,      // trail added per agent per step
    pub decay: f32,        // trail multiplier per step (e.g. 0.97)
}

impl SimParams {
    pub fn default_for(width: u32, height: u32, n_agents: u32) -> Self {
        SimParams {
            width,
            height,
            n_agents,
            frame: 0,
            move_speed: 1.0,
            turn_speed: 0.35,
            sensor_angle: 0.5,
            sensor_dist: 9.0,
            deposit: 1.0,
            decay: 0.97,
        }
    }
}

const TAU: f32 = 2.0 * core::f32::consts::PI;

/// Toroidal wrap of a coordinate into [0, max).
fn wrap(v: f32, max: f32) -> f32 {
    if v < 0.0 {
        v + max
    } else if v >= max {
        v - max
    } else {
        v
    }
}

/// Trail index for a (wrapped) position.
pub fn trail_index(x: f32, y: f32, p: &SimParams) -> u32 {
    let xi = (wrap(x, p.width as f32)) as u32 % p.width;
    let yi = (wrap(y, p.height as f32)) as u32 % p.height;
    yi * p.width + xi
}

/// Sample the trail at `dist` pixels ahead of the agent, `angle_off` radians
/// off its heading.
fn sense(agent: &Agent, angle_off: f32, trail: &[f32], p: &SimParams) -> f32 {
    let a = agent.angle + angle_off;
    let sx = agent.x + a.cos() * p.sensor_dist;
    let sy = agent.y + a.sin() * p.sensor_dist;
    trail[trail_index(sx, sy, p) as usize]
}

/// Spawn agent `i` deterministically: ring around the field center, heading
/// inward — produces a satisfying collapse-then-network opening.
pub fn spawn_agent(i: u32, p: &SimParams) -> Agent {
    let mut state = rng::seed(i, 0xC0FFEE, p.frame);
    let t = rng::next_f32(&mut state) * TAU;
    let r = 0.35 * (p.height as f32) * (0.6 + 0.4 * rng::next_f32(&mut state));
    let cx = p.width as f32 * 0.5;
    let cy = p.height as f32 * 0.5;
    Agent {
        x: cx + t.cos() * r,
        y: cy + t.sin() * r,
        angle: t + core::f32::consts::PI, // face the center
        _pad: 0.0,
    }
}

/// One simulation step for one agent: sense → steer → move → wrap.
/// Pure: returns the new agent; the deposit cell is returned separately so the
/// caller (CPU loop or GPU entry) owns the write.
pub fn update_agent(agent: &Agent, trail: &[f32], p: &SimParams, agent_index: u32) -> (Agent, u32) {
    let left = sense(agent, p.sensor_angle, trail, p);
    let center = sense(agent, 0.0, trail, p);
    let right = sense(agent, -p.sensor_angle, trail, p);

    let mut angle = agent.angle;
    if center >= left && center >= right {
        // hold course
    } else if left > right {
        angle += p.turn_speed;
    } else if right > left {
        angle -= p.turn_speed;
    } else {
        // exact tie, center weakest: pick a side pseudo-randomly but
        // deterministically from (index, frame)
        let mut state = rng::seed(agent_index, p.frame, 0x51AE);
        if rng::next_u32(&mut state) & 1 == 0 {
            angle += p.turn_speed;
        } else {
            angle -= p.turn_speed;
        }
    }

    let x = wrap(agent.x + angle.cos() * p.move_speed, p.width as f32);
    let y = wrap(agent.y + angle.sin() * p.move_speed, p.height as f32);
    let next = Agent {
        x,
        y,
        angle,
        _pad: 0.0,
    };
    let cell = trail_index(x, y, p);
    (next, cell)
}

/// Diffuse + decay for one trail cell: 3×3 toroidal mean, then decay.
pub fn diffuse_at(trail: &[f32], x: u32, y: u32, p: &SimParams) -> f32 {
    let w = p.width;
    let h = p.height;
    let mut sum = 0.0f32;
    let mut dy = 0u32;
    while dy < 3 {
        let mut dx = 0u32;
        while dx < 3 {
            let nx = (x + w + dx - 1) % w;
            let ny = (y + h + dy - 1) % h;
            sum += trail[(ny * w + nx) as usize];
            dx += 1;
        }
        dy += 1;
    }
    (sum / 9.0) * p.decay
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params() -> SimParams {
        SimParams::default_for(64, 48, 8)
    }

    #[test]
    fn spawn_is_deterministic_and_in_bounds() {
        let p = params();
        for i in 0..8 {
            let a = spawn_agent(i, &p);
            let b = spawn_agent(i, &p);
            assert_eq!(a, b);
            assert!(a.x >= 0.0 && a.x < p.width as f32);
            assert!(a.y >= 0.0 && a.y < p.height as f32);
        }
        assert_ne!(spawn_agent(0, &p), spawn_agent(1, &p));
    }

    #[test]
    fn agent_moves_and_wraps() {
        let p = params();
        let trail = vec![0.0f32; (p.width * p.height) as usize];
        let start = Agent {
            x: 63.5,
            y: 10.0,
            angle: 0.0,
            _pad: 0.0,
        };
        let (next, cell) = update_agent(&start, &trail, &p, 0);
        assert!(next.x < 1.0, "should wrap toroidally, got {}", next.x);
        assert_eq!(next.y, 10.0);
        assert!(cell < p.width * p.height);
    }

    #[test]
    fn steers_toward_stronger_trail() {
        let p = params();
        let mut trail = vec![0.0f32; (p.width * p.height) as usize];
        let agent = Agent {
            x: 32.0,
            y: 24.0,
            angle: 0.0,
            _pad: 0.0,
        };
        // pile trail at the LEFT sensor position
        let sx = agent.x + (agent.angle + p.sensor_angle).cos() * p.sensor_dist;
        let sy = agent.y + (agent.angle + p.sensor_angle).sin() * p.sensor_dist;
        trail[trail_index(sx, sy, &p) as usize] = 10.0;
        let (next, _) = update_agent(&agent, &trail, &p, 0);
        assert!(next.angle > agent.angle, "should turn left toward signal");
    }

    #[test]
    fn diffuse_decays_total_mass() {
        let p = params();
        let mut trail = vec![0.0f32; (p.width * p.height) as usize];
        trail[(24 * p.width + 32) as usize] = 9.0;
        let next: Vec<f32> = (0..p.width * p.height)
            .map(|i| diffuse_at(&trail, i % p.width, i / p.width, &p))
            .collect();
        let before: f32 = trail.iter().sum();
        let after: f32 = next.iter().sum();
        // 3x3 mean conserves mass on a torus; decay multiplies by p.decay
        assert!((after - before * p.decay).abs() < 1e-3, "{after} vs {before}");
        // the spike spread to exactly the 3x3 neighborhood
        assert_eq!(next.iter().filter(|v| **v > 0.0).count(), 9);
    }

    #[test]
    fn update_is_deterministic() {
        let p = params();
        let trail = vec![0.5f32; (p.width * p.height) as usize];
        let a = spawn_agent(3, &p);
        assert_eq!(update_agent(&a, &trail, &p, 3), update_agent(&a, &trail, &p, 3));
    }
}
