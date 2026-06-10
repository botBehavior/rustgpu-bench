// Hand-written WGSL twin of the gpu_shared path tracer. Same algorithm,
// same RNG, same scene, same iteration structure — written fresh in
// idiomatic WGSL, not transpiled.

struct RenderParams {
    width: u32,
    height: u32,
    samples: u32,
    seed: u32,
}

@group(0) @binding(0) var<storage, read> params: RenderParams;
@group(0) @binding(1) var<storage, read_write> out_buf: array<f32>;

const PI: f32 = 3.14159265358979;
const MAX_BOUNCES: u32 = 8u;
const T_MIN: f32 = 0.001;
const T_MAX: f32 = 1.0e30;
const NUM_SPHERES: u32 = 4u;

// ---------- RNG: wang hash seed + xorshift32, top-24-bit floats ----------

fn wang_hash(x_in: u32) -> u32 {
    var x = (x_in ^ 61u) ^ (x_in >> 16u);
    x = x * 9u;
    x = x ^ (x >> 4u);
    x = x * 0x27d4eb2du;
    x = x ^ (x >> 15u);
    return x;
}

fn rng_seed(pixel_index: u32, sample: u32, frame_seed: u32) -> u32 {
    let s = wang_hash(pixel_index ^ wang_hash(sample ^ wang_hash(frame_seed)));
    if (s == 0u) {
        return 0x9e3779b9u;
    }
    return s;
}

fn next_u32(state: ptr<function, u32>) -> u32 {
    var x = *state;
    x = x ^ (x << 13u);
    x = x ^ (x >> 17u);
    x = x ^ (x << 5u);
    *state = x;
    return x;
}

fn next_f32(state: ptr<function, u32>) -> f32 {
    return f32(next_u32(state) >> 8u) * (1.0 / 16777216.0);
}

fn random_unit_vector(state: ptr<function, u32>) -> vec3<f32> {
    let z = 1.0 - 2.0 * next_f32(state);
    let phi = 2.0 * PI * next_f32(state);
    let r = sqrt(max(1.0 - z * z, 0.0));
    return vec3<f32>(r * cos(phi), r * sin(phi), z);
}

// ---------- scene ----------

struct Sphere {
    center: vec3<f32>,
    radius: f32,
    albedo: vec3<f32>,
    material: u32, // 0 = lambertian, 1 = metal
    fuzz: f32,
}

fn sphere(i: u32) -> Sphere {
    switch i {
        case 0u: {
            return Sphere(vec3<f32>(0.0, -100.5, -1.0), 100.0, vec3<f32>(0.8, 0.8, 0.0), 0u, 0.0);
        }
        case 1u: {
            return Sphere(vec3<f32>(0.0, 0.0, -1.2), 0.5, vec3<f32>(0.1, 0.2, 0.5), 0u, 0.0);
        }
        case 2u: {
            return Sphere(vec3<f32>(-1.0, 0.0, -1.0), 0.5, vec3<f32>(0.8, 0.8, 0.8), 1u, 0.05);
        }
        default: {
            return Sphere(vec3<f32>(1.0, 0.0, -1.0), 0.5, vec3<f32>(0.8, 0.6, 0.2), 1u, 0.4);
        }
    }
}

struct Hit {
    t: f32,
    point: vec3<f32>,
    normal: vec3<f32>,
    albedo: vec3<f32>,
    material: u32,
    fuzz: f32,
}

fn hit_scene(origin: vec3<f32>, dir: vec3<f32>) -> Hit {
    var closest = Hit(T_MAX, vec3<f32>(), vec3<f32>(), vec3<f32>(), 0u, 0.0);
    for (var i = 0u; i < NUM_SPHERES; i = i + 1u) {
        let s = sphere(i);
        let oc = origin - s.center;
        let a = dot(dir, dir);
        let half_b = dot(oc, dir);
        let c = dot(oc, oc) - s.radius * s.radius;
        let disc = half_b * half_b - a * c;
        if (disc > 0.0) {
            let sqrt_d = sqrt(disc);
            var t = (-half_b - sqrt_d) / a;
            if (t < T_MIN) {
                t = (-half_b + sqrt_d) / a;
            }
            if (t >= T_MIN && t < closest.t) {
                let point = origin + t * dir;
                closest = Hit(t, point, (point - s.center) / s.radius, s.albedo, s.material, s.fuzz);
            }
        }
    }
    return closest;
}

fn sky(dir: vec3<f32>) -> vec3<f32> {
    let t = 0.5 * (normalize(dir).y + 1.0);
    return (1.0 - t) * vec3<f32>(1.0, 1.0, 1.0) + t * vec3<f32>(0.5, 0.7, 1.0);
}

fn ray_color(origin_in: vec3<f32>, dir_in: vec3<f32>, state: ptr<function, u32>) -> vec3<f32> {
    var origin = origin_in;
    var dir = dir_in;
    var throughput = vec3<f32>(1.0, 1.0, 1.0);
    for (var bounce = 0u; bounce < MAX_BOUNCES; bounce = bounce + 1u) {
        let hit = hit_scene(origin, dir);
        if (hit.t >= T_MAX) {
            return throughput * sky(dir);
        }
        let front = dot(dir, hit.normal) < 0.0;
        var normal = hit.normal;
        if (!front) {
            normal = -normal;
        }
        throughput = throughput * hit.albedo;
        origin = hit.point;
        if (hit.material == 0u) {
            var scatter = normal + random_unit_vector(state);
            if (dot(scatter, scatter) < 1.0e-8) {
                scatter = normal;
            }
            dir = scatter;
        } else {
            let reflected = reflect(normalize(dir), normal);
            dir = reflected + hit.fuzz * random_unit_vector(state);
            if (dot(dir, normal) <= 0.0) {
                return vec3<f32>();
            }
        }
    }
    return vec3<f32>();
}

fn render_pixel(px: u32, py: u32) -> vec3<f32> {
    let aspect = f32(params.width) / f32(params.height);
    let viewport_h = 2.0;
    let viewport_w = viewport_h * aspect;

    var acc = vec3<f32>();
    let pixel_index = py * params.width + px;
    for (var s = 0u; s < params.samples; s = s + 1u) {
        var state = rng_seed(pixel_index, s, params.seed);
        let jx = next_f32(&state);
        let jy = next_f32(&state);
        let u = (f32(px) + jx) / f32(params.width);
        let v = 1.0 - (f32(py) + jy) / f32(params.height);
        let dir = vec3<f32>((u - 0.5) * viewport_w, (v - 0.5) * viewport_h, -1.0);
        acc = acc + ray_color(vec3<f32>(), dir, &state);
    }
    return acc / f32(params.samples);
}

@compute @workgroup_size(8, 8)
fn render_cs(@builtin(global_invocation_id) id: vec3<u32>) {
    let x = id.x;
    let y = id.y;
    if (x >= params.width || y >= params.height) {
        return;
    }
    let c = render_pixel(x, y);
    let base = (y * params.width + x) * 3u;
    out_buf[base] = c.x;
    out_buf[base + 1u] = c.y;
    out_buf[base + 2u] = c.z;
}
