// CORRECTION 2026-06-10: NVIDIA Vulkan DRIVER miscompile, not rust-gpu (the nested select
// `if ..<(745348641|y){x}else{..}` reproduces from hand-WGSL; correct on WARP/CPU). The
// outer batch wrapping is incidental. See conformance/NOTES.md Finding 1.
// MISMATCH seed=303 fn=44 x=1518255298 y=3271939385
// cpu(interp)=2125137920 gpu=264744960 (gpu wrong = NVIDIA driver)
// verify natively: this file is valid Rust
pub fn repro(x: u32, y: u32) -> u32 { (({ let mut acc = (((y.wrapping_sub(y)) << ((y >> (y & 31)) & 31)) << (((96785975u32.wrapping_add(48193735u32)).wrapping_mul((y << (x & 31)))) & 31)); let mut i = 0u32; while i < 4u32 { acc = acc.wrapping_mul(1664525).wrapping_add((y.wrapping_add((if (if x < x { 396779213u32 } else { y }) < (745348641u32 | y) { x } else { (if x < 3850752092u32 { x } else { y }) })))); i += 1; } acc }) << (3766287817u32 & 31)) }
fn main() { assert_eq!(repro(1518255298, 3271939385), 2125137920u32, "native rustc disagrees with interpreter"); }
