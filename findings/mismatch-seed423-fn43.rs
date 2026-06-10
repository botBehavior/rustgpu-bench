// MISMATCH seed=423 fn=43 x=1717998298 y=2775663729
// cpu(interp)=545966273 gpu=25839809
// verify natively: this file is valid Rust
pub fn repro(x: u32, y: u32) -> u32 { (835635425u32 & ({ let mut acc = (if (x | (y.wrapping_sub(y))) < x { ((y << (x & 31)) & ({ let mut acc = 695379700u32; let mut i = 0u32; while i < 8u32 { acc = acc.wrapping_mul(1664525).wrapping_add(y); i += 1; } acc })) } else { y }); let mut i = 0u32; while i < 8u32 { acc = acc.wrapping_mul(1664525).wrapping_add((((if x < 1114505204u32 { y } else { y }).wrapping_add((if x < x { y } else { 1629422154u32 }))) | (if (447885322u32 >> (3549272988u32 & 31)) < (x.wrapping_mul(4137330452u32)) { y } else { (y % (3838954833u32 | 1)) }))); i += 1; } acc })) }
fn main() { assert_eq!(repro(1717998298, 2775663729), 545966273u32, "native rustc disagrees with interpreter"); }
