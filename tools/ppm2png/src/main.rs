//! Minimal P6 PPM → PNG converter for README assets.
//! Usage: ppm2png <in.ppm> <out.png>

use std::io::BufWriter;

fn main() {
    let mut args = std::env::args().skip(1);
    let inp = args.next().expect("usage: ppm2png <in.ppm> <out.png>");
    let out = args.next().expect("usage: ppm2png <in.ppm> <out.png>");
    let bytes = std::fs::read(&inp).expect("read ppm");

    // header: "P6\n<w> <h>\n255\n"
    let header_end = bytes
        .windows(4)
        .position(|w| w == b"255\n")
        .expect("P6 header")
        + 4;
    let header = std::str::from_utf8(&bytes[..header_end]).expect("ascii header");
    let mut nums = header
        .split_whitespace()
        .skip(1)
        .filter_map(|t| t.parse::<u32>().ok());
    let (w, h) = (nums.next().unwrap(), nums.next().unwrap());

    let file = std::fs::File::create(&out).expect("create png");
    let mut enc = png::Encoder::new(BufWriter::new(file), w, h);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    let mut writer = enc.write_header().expect("png header");
    writer
        .write_image_data(&bytes[header_end..header_end + (w * h * 3) as usize])
        .expect("png data");
    println!("wrote {out} ({w}x{h})");
}
