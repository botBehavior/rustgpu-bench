//! Shader math as ordinary Rust.
//!
//! Every function here is `no_std`-compatible, SPIR-V-subset-safe (no checked
//! arithmetic, no recursion, bounded loops, no `usize` in data), runs under
//! `cargo test` on the CPU, and compiles unchanged to SPIR-V via rust-gpu.
//! That last sentence is the whole point: this is shader code with rustdoc,
//! unit tests, and a borrow checker.
#![cfg_attr(target_arch = "spirv", no_std)]

pub mod color;
pub mod gallery;
pub mod noise;
pub mod sdf2;
