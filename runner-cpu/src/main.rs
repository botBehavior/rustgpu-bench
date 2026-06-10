//! CPU-truth renderer: runs the shared path-tracer kernel on all cores via
//! rayon and writes a PPM. This output is the oracle the GPU arm is compared
//! against in M2.
//!
//! Usage: runner-cpu [width height samples] (default 800 450 32)

use rayon::prelude::*;
use std::io::Write;
use std::time::Instant;

use gpu_shared::{render_pixel, RenderParams};

fn main() {
    let args: Vec<u32> = std::env::args()
        .skip(1)
        .filter_map(|a| a.parse().ok())
        .collect();
    let params = RenderParams {
        width: *args.first().unwrap_or(&800),
        height: *args.get(1).unwrap_or(&450),
        samples: *args.get(2).unwrap_or(&32),
        seed: 7,
    };

    let start = Instant::now();
    let pixels: Vec<_> = (0..params.height * params.width)
        .into_par_iter()
        .map(|i| render_pixel(i % params.width, i / params.width, &params))
        .collect();
    let elapsed = start.elapsed();
    println!(
        "CPU ({} threads): {}x{} @ {} spp in {:.1} ms",
        rayon::current_num_threads(),
        params.width,
        params.height,
        params.samples,
        elapsed.as_secs_f64() * 1000.0
    );

    let out_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../out-cpu.ppm");
    let mut f = std::io::BufWriter::new(std::fs::File::create(&out_path).unwrap());
    writeln!(f, "P6\n{} {}\n255", params.width, params.height).unwrap();
    for c in &pixels {
        // gamma 2.0 (sqrt), matching what the GPU presentation will do
        for ch in [c.x, c.y, c.z] {
            let g = ch.max(0.0).sqrt().min(1.0);
            f.write_all(&[(g * 255.999) as u8]).unwrap();
        }
    }
    println!("wrote {}", out_path.display());
}
