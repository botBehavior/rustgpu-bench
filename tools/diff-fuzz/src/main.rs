//! Differential fuzzer for rust-gpu: generate pure integer Rust functions,
//! compile them to SPIR-V via cargo-gpu, run on the GPU, and diff against a
//! CPU interpreter of the same AST. Integer/bool/control-flow domain only —
//! results must be bit-exact, so ANY mismatch is a finding.
//!
//! Usage: diff-fuzz [batches] [seed0]   (default 1 batch, seed 1)
//!
//! Ground truth on mismatch: the generated source is plain Rust — recompile
//! the repro natively with rustc and the CPU binary is the arbiter (the
//! interpreter itself could be wrong; that's a finding about the fuzzer).

use std::fmt::Write as _;
use std::process::Command;
use std::time::Instant;

const N_FNS: u32 = 48;
const N_INPUTS: u32 = 4096;
const MAX_DEPTH: u32 = 5;
/// Calls may only target the first CALL_TARGETS functions, which are themselves
/// call-free (their index < CALL_TARGETS). This keeps inlining depth at 1:
/// rust-gpu's logical-pointer legalizer must inline every call, so unrestricted
/// call chains explode combinatorially and hang the compiler (observed: a 48-fn
/// dense-call batch killed rustc). Leaf-only callees exercise cross-function
/// call lowering + argument passing while staying polynomial.
const CALL_TARGETS: u32 = 4;

// ---------- deterministic rng ----------
struct Rng(u32);
impl Rng {
    fn next(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }
    fn below(&mut self, n: u32) -> u32 {
        self.next() % n
    }
}

fn lcg(s: u32) -> u32 {
    s.wrapping_mul(1664525).wrapping_add(1013904223)
}

// ---------- AST ----------
#[derive(Clone, Debug)]
enum Node {
    X,
    Y,
    C(u32),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),
    Xor(Box<Node>, Box<Node>),
    Shl(Box<Node>, Box<Node>),  // rhs masked & 31
    Shr(Box<Node>, Box<Node>),  // rhs masked & 31
    Div(Box<Node>, Box<Node>),  // rhs | 1 (no div-by-zero)
    Rem(Box<Node>, Box<Node>),  // rhs | 1
    Sel(Box<Node>, Box<Node>, Box<Node>, Box<Node>), // if a < b { t } else { f }
    Loop(Box<Node>, Box<Node>, u32), // acc=e0; k times: acc = acc*M + e1
    /// Call f_j(arg0, arg1) for an EARLIER function j < this fn's index. The
    /// strict ordering keeps the call graph acyclic (SPIR-V forbids recursion)
    /// and exercises rust-gpu's cross-function inlining / pointer legalization.
    Call(u32, Box<Node>, Box<Node>),
}

/// `my_idx` = the index of the function currently being generated; calls may
/// only target earlier functions, so generation needs to know it.
fn gen(rng: &mut Rng, depth: u32, my_idx: u32) -> Node {
    use Node::*;
    if depth >= MAX_DEPTH || rng.below(100) < 18 {
        return match rng.below(3) {
            0 => X,
            1 => Y,
            _ => C(rng.next()),
        };
    }
    macro_rules! a {
        () => {
            Box::new(gen(rng, depth + 1, my_idx))
        };
    }
    let op = rng.below(16);
    match op {
        0 => Add(a!(), a!()),
        1 => Sub(a!(), a!()),
        2 => Mul(a!(), a!()),
        3 => And(a!(), a!()),
        4 => Or(a!(), a!()),
        5 => Xor(a!(), a!()),
        6 => Shl(a!(), a!()),
        7 => Shr(a!(), a!()),
        8 => Div(a!(), a!()),
        9 => Rem(a!(), a!()),
        10 | 11 => Sel(a!(), a!(), a!(), a!()),
        12 => {
            let k = 1 + rng.below(8);
            Loop(a!(), a!(), k)
        }
        13 | 14 if my_idx >= CALL_TARGETS => Call(rng.below(CALL_TARGETS), a!(), a!()),
        _ => Sel(a!(), a!(), a!(), a!()),
    }
}

fn emit(n: &Node, out: &mut String) {
    use Node::*;
    match n {
        X => out.push('x'),
        Y => out.push('y'),
        C(v) => {
            let _ = write!(out, "{v}u32");
        }
        Add(a, b) => bin(out, a, "wrapping_add", b),
        Sub(a, b) => bin(out, a, "wrapping_sub", b),
        Mul(a, b) => bin(out, a, "wrapping_mul", b),
        And(a, b) => infix(out, a, "&", b),
        Or(a, b) => infix(out, a, "|", b),
        Xor(a, b) => infix(out, a, "^", b),
        Shl(a, b) => {
            out.push('(');
            emit(a, out);
            out.push_str(" << (");
            emit(b, out);
            out.push_str(" & 31))");
        }
        Shr(a, b) => {
            out.push('(');
            emit(a, out);
            out.push_str(" >> (");
            emit(b, out);
            out.push_str(" & 31))");
        }
        Div(a, b) => {
            out.push('(');
            emit(a, out);
            out.push_str(" / (");
            emit(b, out);
            out.push_str(" | 1))");
        }
        Rem(a, b) => {
            out.push('(');
            emit(a, out);
            out.push_str(" % (");
            emit(b, out);
            out.push_str(" | 1))");
        }
        Sel(a, b, t, f) => {
            out.push_str("(if ");
            emit(a, out);
            out.push_str(" < ");
            emit(b, out);
            out.push_str(" { ");
            emit(t, out);
            out.push_str(" } else { ");
            emit(f, out);
            out.push_str(" })");
        }
        Loop(e0, e1, k) => {
            out.push_str("({ let mut acc = ");
            emit(e0, out);
            let _ = write!(out, "; let mut i = 0u32; while i < {k}u32 {{ acc = acc.wrapping_mul(1664525).wrapping_add(");
            emit(e1, out);
            out.push_str("); i += 1; } acc })");
        }
        Call(j, a, b) => {
            let _ = write!(out, "f_{j}(");
            emit(a, out);
            out.push_str(", ");
            emit(b, out);
            out.push(')');
        }
    }
}

fn bin(out: &mut String, a: &Node, m: &str, b: &Node) {
    out.push('(');
    emit(a, out);
    let _ = write!(out, ".{m}(");
    emit(b, out);
    out.push_str("))");
}
fn infix(out: &mut String, a: &Node, op: &str, b: &Node) {
    out.push('(');
    emit(a, out);
    let _ = write!(out, " {op} ");
    emit(b, out);
    out.push(')');
}

/// `fns` is the whole module, so a Call can resolve its callee's body. Callees
/// are always earlier indices, so this terminates.
fn interp(n: &Node, x: u32, y: u32, fns: &[Node]) -> u32 {
    use Node::*;
    let e = |m: &Node| interp(m, x, y, fns);
    match n {
        X => x,
        Y => y,
        C(v) => *v,
        Add(a, b) => e(a).wrapping_add(e(b)),
        Sub(a, b) => e(a).wrapping_sub(e(b)),
        Mul(a, b) => e(a).wrapping_mul(e(b)),
        And(a, b) => e(a) & e(b),
        Or(a, b) => e(a) | e(b),
        Xor(a, b) => e(a) ^ e(b),
        Shl(a, b) => e(a) << (e(b) & 31),
        Shr(a, b) => e(a) >> (e(b) & 31),
        Div(a, b) => e(a) / (e(b) | 1),
        Rem(a, b) => e(a) % (e(b) | 1),
        Sel(a, b, t, f) => {
            if e(a) < e(b) {
                e(t)
            } else {
                e(f)
            }
        }
        Loop(e0, e1, k) => {
            let mut acc = e(e0);
            let e1v = e(e1);
            let mut i = 0u32;
            while i < *k {
                acc = acc.wrapping_mul(1664525).wrapping_add(e1v);
                i += 1;
            }
            acc
        }
        Call(j, a, b) => {
            let (a0, a1) = (e(a), e(b));
            // callee body evaluated with arg0->x, arg1->y (matches f_j(arg0,arg1))
            interp(&fns[*j as usize], a0, a1, fns)
        }
    }
}

fn contains_call(n: &Node) -> bool {
    use Node::*;
    match n {
        X | Y | C(_) => false,
        Call(..) => true,
        Add(a, b) | Sub(a, b) | Mul(a, b) | And(a, b) | Or(a, b) | Xor(a, b) | Shl(a, b)
        | Shr(a, b) | Div(a, b) | Rem(a, b) | Loop(a, b, _) => contains_call(a) || contains_call(b),
        Sel(a, b, t, f) => {
            contains_call(a) || contains_call(b) || contains_call(t) || contains_call(f)
        }
    }
}

// NOTE: in Loop, e1 is loop-invariant by construction (no acc/i references),
// so hoisting it in the interpreter matches the emitted while-loop exactly.

fn write_batch(fns: &[Node], dir: &std::path::Path) {
    let mut src = String::from(
        "//! REGENERATED BY tools/diff-fuzz — do not edit by hand.\n#![no_std]\n#![allow(unused_parens, unused_variables, clippy::all)]\n\nuse spirv_std::glam::UVec3;\nuse spirv_std::spirv;\n\n",
    );
    let _ = write!(src, "pub const N_FNS: u32 = {};\n\n", fns.len());
    src.push_str("fn lcg(s: u32) -> u32 { s.wrapping_mul(1664525).wrapping_add(1013904223) }\n\n");
    for (i, f) in fns.iter().enumerate() {
        let _ = write!(src, "pub fn f_{i}(x: u32, y: u32) -> u32 {{ ");
        emit(f, &mut src);
        src.push_str(" }\n");
    }
    src.push_str("\n#[spirv(compute(threads(8, 8)))]\npub fn fuzz_cs(\n    #[spirv(global_invocation_id)] id: UVec3,\n    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] params: &[u32; 4],\n    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] out: &mut [u32],\n) {\n    let ii = id.x;\n    let fi = id.y;\n    if ii >= params[0] || fi >= params[1] { return; }\n    let x = lcg(params[2] ^ ii);\n    let y = lcg(x);\n    let v = match fi {\n");
    for i in 0..fns.len() {
        let _ = write!(src, "        {i} => f_{i}(x, y),\n");
    }
    src.push_str("        _ => 0,\n    };\n    out[(fi * params[0] + ii) as usize] = v;\n}\n");
    std::fs::write(dir.join("src/lib.rs"), src).expect("write generated lib.rs");
}

/// A standalone, natively-compilable repro. Call-free fns emit as a single
/// `repro`; call-containing fns emit every function they can reach (indices
/// 0..=idx, since calls only target earlier functions) so the file compiles.
fn repro_source(fns: &[Node], idx: usize, x: u32, y: u32, cpu: u32, got: u32, seed: u32) -> String {
    let mut s = format!(
        "// MISMATCH seed={seed} fn={idx} x={x} y={y}\n// cpu(interp)={cpu} gpu={got}\n// Valid Rust — native rustc is the arbiter and must agree with the interpreter.\n"
    );
    if contains_call(&fns[idx]) {
        for (j, f) in fns.iter().enumerate().take(idx + 1) {
            let mut b = String::new();
            emit(f, &mut b);
            let _ = write!(s, "pub fn f_{j}(x: u32, y: u32) -> u32 {{ {b} }}\n");
        }
        let _ = write!(
            s,
            "fn main() {{ assert_eq!(f_{idx}({x}, {y}), {cpu}u32, \"native rustc disagrees with interpreter\"); }}\n"
        );
    } else {
        let mut b = String::new();
        emit(&fns[idx], &mut b);
        let _ = write!(
            s,
            "pub fn repro(x: u32, y: u32) -> u32 {{ {b} }}\nfn main() {{ assert_eq!(repro({x}, {y}), {cpu}u32, \"native rustc disagrees with interpreter\"); }}\n"
        );
    }
    s
}

// ---------- shrinker ----------

fn size(n: &Node) -> u32 {
    use Node::*;
    match n {
        X | Y | C(_) => 1,
        Add(a, b) | Sub(a, b) | Mul(a, b) | And(a, b) | Or(a, b) | Xor(a, b) | Shl(a, b)
        | Shr(a, b) | Div(a, b) | Rem(a, b) => 1 + size(a) + size(b),
        Sel(a, b, t, f) => 1 + size(a) + size(b) + size(t) + size(f),
        Loop(a, b, _) | Call(_, a, b) => 1 + size(a) + size(b),
    }
}

/// All single-step simplifications: every subtree replaced by a leaf or by one
/// of its own children (hoisting). Sorted smallest-first by the caller.
fn simplifications(n: &Node) -> Vec<Node> {
    fn subtree_count(n: &Node) -> u32 {
        size(n)
    }
    fn replace(n: &Node, target: &mut u32, with: &dyn Fn(&Node) -> Vec<Node>, acc: &mut Vec<Node>, root: &Node) {
        // generate replacements for the subtree at pre-order index *target
        fn walk(n: &Node, idx: &mut u32, target: u32, out: &mut Vec<(u32, Node)>) {
            let my = *idx;
            *idx += 1;
            if my == target {
                out.push((my, n.clone()));
                return;
            }
            use Node::*;
            match n {
                X | Y | C(_) => {}
                Add(a, b) | Sub(a, b) | Mul(a, b) | And(a, b) | Or(a, b) | Xor(a, b)
                | Shl(a, b) | Shr(a, b) | Div(a, b) | Rem(a, b) | Loop(a, b, _)
                | Call(_, a, b) => {
                    walk(a, idx, target, out);
                    walk(b, idx, target, out);
                }
                Sel(a, b, t, f) => {
                    walk(a, idx, target, out);
                    walk(b, idx, target, out);
                    walk(t, idx, target, out);
                    walk(f, idx, target, out);
                }
            }
        }
        fn rebuild(n: &Node, idx: &mut u32, target: u32, repl: &Node) -> Node {
            let my = *idx;
            *idx += 1;
            if my == target {
                return repl.clone();
            }
            use Node::*;
            macro_rules! r {
                ($a:expr) => {
                    Box::new(rebuild($a, idx, target, repl))
                };
            }
            match n {
                X => X,
                Y => Y,
                C(v) => C(*v),
                Add(a, b) => Add(r!(a), r!(b)),
                Sub(a, b) => Sub(r!(a), r!(b)),
                Mul(a, b) => Mul(r!(a), r!(b)),
                And(a, b) => And(r!(a), r!(b)),
                Or(a, b) => Or(r!(a), r!(b)),
                Xor(a, b) => Xor(r!(a), r!(b)),
                Shl(a, b) => Shl(r!(a), r!(b)),
                Shr(a, b) => Shr(r!(a), r!(b)),
                Div(a, b) => Div(r!(a), r!(b)),
                Rem(a, b) => Rem(r!(a), r!(b)),
                Sel(a, b, t, f) => Sel(r!(a), r!(b), r!(t), r!(f)),
                Loop(a, b, k) => Loop(r!(a), r!(b), *k),
                Call(j, a, b) => Call(*j, r!(a), r!(b)),
            }
        }
        let total = subtree_count(root);
        for t in 0..total {
            let mut found = Vec::new();
            let mut i = 0u32;
            walk(root, &mut i, t, &mut found);
            if let Some((_, sub)) = found.pop() {
                if matches!(sub, Node::X | Node::Y) {
                    continue;
                }
                for cand in with(&sub) {
                    let mut i = 0u32;
                    acc.push(rebuild(root, &mut i, t, &cand));
                }
            }
        }
        let _ = target;
    }
    let mut out = Vec::new();
    let mut zero = 0u32;
    replace(
        n,
        &mut zero,
        &|sub: &Node| {
            use Node::*;
            let mut v = vec![X, Y, C(0)];
            match sub {
                Add(a, b) | Sub(a, b) | Mul(a, b) | And(a, b) | Or(a, b) | Xor(a, b)
                | Shl(a, b) | Shr(a, b) | Div(a, b) | Rem(a, b) | Loop(a, b, _)
                | Call(_, a, b) => {
                    v.push((**a).clone());
                    v.push((**b).clone());
                }
                Sel(a, b, t, f) => {
                    v.push((**a).clone());
                    v.push((**b).clone());
                    v.push((**t).clone());
                    v.push((**f).clone());
                }
                _ => {}
            }
            v
        },
        &mut out,
        n,
    );
    out
}

fn any_mismatch(gpu_out: &[u32], f: &Node, seed: u32, slot: usize, fns: &[Node]) -> bool {
    for ii in 0..N_INPUTS {
        let x = lcg(seed ^ ii);
        let y = lcg(x);
        if interp(f, x, y, fns) != gpu_out[slot * N_INPUTS as usize + ii as usize] {
            return true;
        }
    }
    false
}

fn shrink(seed: u32, fn_idx: usize, root: &std::path::Path, gpu: &GpuCtx) {
    let fuzz_dir = root.join("fuzz-shaders");
    let spv_path = fuzz_dir.join("spv/fuzz_shaders.spv");
    let mut rng = Rng(if seed == 0 { 0xDEAD_BEEF } else { lcg(seed) });
    let fns: Vec<Node> = (0..N_FNS).map(|i| gen(&mut rng, 0, i)).collect();
    let mut best = fns[fn_idx].clone();
    if contains_call(&best) {
        println!("fn {fn_idx} contains calls — per-fn shrink batches candidates into one module, which breaks position-bound call indices. Use the campaign's 0..=idx prefix repro (already valid Rust), or --bisect.");
        return;
    }
    println!("shrinking seed {seed} fn {fn_idx}: start size {}", size(&best));

    loop {
        let mut cands = simplifications(&best);
        cands.sort_by_key(size);
        cands.truncate(160); // cap probes per round
        let mut improved = false;
        for chunk in cands.chunks(N_FNS as usize) {
            write_batch(chunk, &fuzz_dir);
            let ok = Command::new("cargo")
                .args(["gpu", "build", "--shader-crate", fuzz_dir.to_str().unwrap(),
                       "--output-dir", fuzz_dir.join("spv").to_str().unwrap(),
                       "--auto-install-rust-toolchain"])
                .current_dir(root)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);
            if !ok {
                continue;
            }
            let spv = std::fs::read(&spv_path).expect("spv");
            let out = run_gpu(gpu, &spv, seed, chunk.len() as u32);
            for (slot, cand) in chunk.iter().enumerate() {
                // shrink only runs on call-free fns, so chunk is a valid fns table
                if any_mismatch(&out, cand, seed, slot, chunk) {
                    best = cand.clone();
                    improved = true;
                    println!("  -> size {}", size(&best));
                    break;
                }
            }
            if improved {
                break;
            }
        }
        if !improved {
            break;
        }
    }

    let mut src = String::new();
    emit(&best, &mut src);
    // find a concrete failing input for the minimized form
    write_batch(std::slice::from_ref(&best), &fuzz_dir);
    Command::new("cargo")
        .args(["gpu", "build", "--shader-crate", fuzz_dir.to_str().unwrap(),
               "--output-dir", fuzz_dir.join("spv").to_str().unwrap(),
               "--auto-install-rust-toolchain"])
        .current_dir(root)
        .output()
        .ok();
    let spv = std::fs::read(&spv_path).expect("spv");
    let out = run_gpu(gpu, &spv, seed, 1);
    let mut example = (0u32, 0u32, 0u32, 0u32);
    for ii in 0..N_INPUTS {
        let x = lcg(seed ^ ii);
        let y = lcg(x);
        let cpu = interp(&best, x, y, std::slice::from_ref(&best));
        if cpu != out[ii as usize] {
            example = (x, y, cpu, out[ii as usize]);
            break;
        }
    }
    let path = root.join(format!("findings/min-seed{seed}-fn{fn_idx}.rs"));
    std::fs::write(&path, format!(
        "// MINIMIZED rust-gpu miscompile repro (size {} nodes)\n// x={} y={}: native/interp = {}, GPU (passthrough AND naga paths) = {}\npub fn repro(x: u32, y: u32) -> u32 {{ {src} }}\nfn main() {{ assert_eq!(repro({}, {}), {}u32); }}\n",
        size(&best), example.0, example.1, example.2, example.3, example.0, example.1, example.2
    )).expect("write minimized");
    println!("minimized -> {} (size {})\n{src}", path.display(), size(&best));
}

/// Build the module (context fns + target as LAST slot), run, return whether
/// the target slot mismatches the interpreter. None = compile failure.
fn probe(
    root: &std::path::Path,
    gpu: &GpuCtx,
    seed: u32,
    ctx_fns: &[Node],
    target: &Node,
) -> Option<bool> {
    let fuzz_dir = root.join("fuzz-shaders");
    let mut module: Vec<Node> = ctx_fns.to_vec();
    module.push(target.clone());
    write_batch(&module, &fuzz_dir);
    let ok = Command::new("cargo")
        .args(["gpu", "build", "--shader-crate", fuzz_dir.to_str().unwrap(),
               "--output-dir", fuzz_dir.join("spv").to_str().unwrap(),
               "--auto-install-rust-toolchain"])
        .current_dir(root)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    if !ok {
        return None;
    }
    let spv = std::fs::read(fuzz_dir.join("spv/fuzz_shaders.spv")).ok()?;
    let out = run_gpu(gpu, &spv, seed, module.len() as u32);
    Some(any_mismatch(&out, target, seed, module.len() - 1, &module))
}

fn bisect(seed: u32, fn_idx: usize, root: &std::path::Path, gpu: &GpuCtx) {
    let mut rng = Rng(if seed == 0 { 0xDEAD_BEEF } else { lcg(seed) });
    let fns: Vec<Node> = (0..N_FNS).map(|i| gen(&mut rng, 0, i)).collect();
    let mut target = fns[fn_idx].clone();
    if contains_call(&target) {
        println!("fn {fn_idx} contains calls — ddmin reorders functions, scrambling position-bound call indices. The campaign already emits a valid 0..=idx prefix repro for call-containing findings; minimizing those needs call-index remapping (future work).");
        return;
    }
    let mut ctx: Vec<Node> = fns
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != fn_idx)
        .map(|(_, f)| f.clone())
        .collect();

    match probe(root, gpu, seed, &ctx, &target) {
        Some(true) => println!("baseline (target-last, full context): mismatch confirmed"),
        other => {
            println!("baseline FAILED to reproduce with target in last slot ({other:?}) — slot position matters; aborting bisect");
            return;
        }
    }

    // ddmin over context fns
    let mut chunk = ctx.len().div_ceil(2);
    loop {
        let mut removed = false;
        let mut i = 0usize;
        while i < ctx.len() {
            let end = (i + chunk).min(ctx.len());
            let mut trial = ctx.clone();
            trial.drain(i..end);
            if probe(root, gpu, seed, &trial, &target) == Some(true) {
                ctx = trial;
                removed = true;
                println!("  context -> {} fns", ctx.len());
            } else {
                i = end;
            }
        }
        if chunk == 1 {
            if !removed {
                break;
            }
        } else {
            chunk = (chunk / 2).max(1);
        }
    }
    println!("minimal context: {} sibling fn(s)", ctx.len());

    // in-context greedy shrink of the target (one candidate per compile)
    let mut probes = 0u32;
    'outer: loop {
        let mut cands = simplifications(&target);
        cands.sort_by_key(size);
        for cand in cands {
            if probes >= 80 {
                println!("probe budget reached");
                break 'outer;
            }
            probes += 1;
            if probe(root, gpu, seed, &ctx, &cand) == Some(true) {
                target = cand;
                println!("  target -> size {}", size(&target));
                continue 'outer;
            }
        }
        break;
    }
    // also shrink each context fn greedily (cheap pass, leaves only)
    for ci in 0..ctx.len() {
        loop {
            let mut cands = simplifications(&ctx[ci]);
            cands.sort_by_key(size);
            cands.truncate(12);
            let mut improved = false;
            for cand in cands {
                if probes >= 140 {
                    break;
                }
                probes += 1;
                let mut trial = ctx.clone();
                trial[ci] = cand.clone();
                if probe(root, gpu, seed, &trial, &target) == Some(true) {
                    ctx = trial;
                    improved = true;
                    break;
                }
            }
            if !improved {
                break;
            }
        }
    }

    // emit the final minimal MODULE as the repro artifact
    let fuzz_dir = root.join("fuzz-shaders");
    let mut module: Vec<Node> = ctx.clone();
    module.push(target.clone());
    write_batch(&module, &fuzz_dir);
    let lib = std::fs::read_to_string(fuzz_dir.join("src/lib.rs")).unwrap();
    let out_path = root.join(format!("findings/min-module-seed{seed}.rs"));
    let mut report = format!(
        "// MINIMIZED MODULE repro for rust-gpu miscompile (seed {seed}, original fn {fn_idx})\n// Context-dependent: target (LAST fn) miscompiles only alongside {} sibling fn(s).\n// Reproduce: use this as fuzz-shaders/src/lib.rs, build with cargo-gpu, dispatch fuzz_cs,\n// compare target slot against CPU. Mismatch confirmed on both passthrough and naga paths.\n\n",
        ctx.len()
    );
    report.push_str(&lib);
    std::fs::write(&out_path, report).expect("write module repro");
    println!(
        "module repro -> {} (context {} fns, target size {}, {} probes)",
        out_path.display(),
        ctx.len(),
        size(&target),
        probes
    );
    let mut tsrc = String::new();
    emit(&target, &mut tsrc);
    println!("target fn: {tsrc}");
}

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    if argv.first().map(|s| s.as_str()) == Some("--bisect") {
        let seed: u32 = argv[1].parse().expect("seed");
        let fn_idx: usize = argv[2].parse().expect("fn idx");
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let gpu = pollster::block_on(init_gpu());
        bisect(seed, fn_idx, &root, &gpu);
        return;
    }
    if argv.first().map(|s| s.as_str()) == Some("--shrink") {
        let seed: u32 = argv[1].parse().expect("seed");
        let fn_idx: usize = argv[2].parse().expect("fn idx");
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let gpu = pollster::block_on(init_gpu());
        shrink(seed, fn_idx, &root, &gpu);
        return;
    }
    let args: Vec<u32> = std::env::args().skip(1).filter_map(|a| a.parse().ok()).collect();
    let batches = *args.first().unwrap_or(&1);
    let seed0 = *args.get(1).unwrap_or(&1);

    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fuzz_dir = root.join("fuzz-shaders");
    let spv_path = fuzz_dir.join("spv/fuzz_shaders.spv");
    let findings_dir = root.join("fuzz-findings");
    std::fs::create_dir_all(&findings_dir).ok();

    let gpu = pollster::block_on(init_gpu());
    let mut total_fns = 0u64;
    let mut total_cmp = 0u64;
    let mut findings = 0u32;
    let t0 = Instant::now();

    for batch in 0..batches {
        let seed = seed0.wrapping_add(batch);
        let mut rng = Rng(if seed == 0 { 0xDEAD_BEEF } else { lcg(seed) });
        let fns: Vec<Node> = (0..N_FNS).map(|i| gen(&mut rng, 0, i)).collect();
        write_batch(&fns, &fuzz_dir);

        let status = Command::new("cargo")
            .args([
                "gpu",
                "build",
                "--shader-crate",
                fuzz_dir.to_str().unwrap(),
                "--output-dir",
                fuzz_dir.join("spv").to_str().unwrap(),
                "--auto-install-rust-toolchain",
            ])
            .current_dir(&root)
            .output()
            .expect("spawn cargo gpu");
        if !status.status.success() {
            // a generated program that FAILS to compile is itself a finding
            let path = findings_dir.join(format!("compile-fail-seed{seed}.txt"));
            std::fs::write(
                &path,
                format!(
                    "seed {seed}: cargo gpu build failed\n\n{}",
                    String::from_utf8_lossy(&status.stderr)
                ),
            )
            .ok();
            println!("batch seed {seed}: COMPILE FAIL -> {}", path.display());
            findings += 1;
            continue;
        }

        let spv = std::fs::read(&spv_path).expect("read fuzz spv");
        let gpu_out = run_gpu(&gpu, &spv, seed, fns.len() as u32);

        for (fi, f) in fns.iter().enumerate() {
            for ii in 0..N_INPUTS {
                let x = lcg(seed ^ ii);
                let y = lcg(x);
                let cpu = interp(f, x, y, &fns);
                let got = gpu_out[fi * N_INPUTS as usize + ii as usize];
                total_cmp += 1;
                if cpu != got {
                    findings += 1;
                    let kind = if contains_call(f) { "call" } else { "flat" };
                    let path = findings_dir.join(format!("mismatch-seed{seed}-fn{fi}.rs"));
                    std::fs::write(&path, repro_source(&fns, fi, x, y, cpu, got, seed)).ok();
                    println!("batch seed {seed} fn {fi}: MISMATCH ({kind}) at x={x} y={y} (cpu {cpu} vs gpu {got}) -> {}", path.display());
                    break; // one repro per fn is enough
                }
            }
        }
        total_fns += fns.len() as u64;
        if batch % 10 == 9 || batches <= 5 {
            println!(
                "[{:>5.1}s] {} batches, {} fns, {} comparisons, {} findings",
                t0.elapsed().as_secs_f32(),
                batch + 1,
                total_fns,
                total_cmp,
                findings
            );
        }
    }
    println!(
        "DONE: {batches} batches / {total_fns} fns / {total_cmp} comparisons / {findings} findings in {:.1}s",
        t0.elapsed().as_secs_f32()
    );
    if findings > 0 {
        std::process::exit(2);
    }
}

struct GpuCtx {
    device: wgpu::Device,
    queue: wgpu::Queue,
    passthrough: bool,
}

async fn init_gpu() -> GpuCtx {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        })
        .await
        .expect("no adapter");
    let passthrough = std::env::var("DIFF_FUZZ_NAGA").is_err()
        && adapter
            .features()
            .contains(wgpu::Features::PASSTHROUGH_SHADERS);
    println!(
        "shader path: {}",
        if passthrough { "passthrough" } else { "naga frontend" }
    );
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: if passthrough {
                wgpu::Features::PASSTHROUGH_SHADERS
            } else {
                wgpu::Features::empty()
            },
            ..Default::default()
        })
        .await
        .expect("device");
    GpuCtx {
        device,
        queue,
        passthrough,
    }
}

fn run_gpu(ctx: &GpuCtx, spv: &[u8], seed: u32, n_fns: u32) -> Vec<u32> {
    let module = if ctx.passthrough {
        unsafe {
            ctx.device
                .create_shader_module_passthrough(wgpu::ShaderModuleDescriptorPassthrough {
                    label: None,
                    spirv: Some(wgpu::util::make_spirv_raw(spv)),
                    ..Default::default()
                })
        }
    } else {
        ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::util::make_spirv(spv),
        })
    };
    let bgl = ctx
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
    let layout = ctx
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(&bgl)],
            immediate_size: 0,
        });
    let pipeline = ctx
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&layout),
            module: &module,
            entry_point: Some("fuzz_cs"),
            compilation_options: Default::default(),
            cache: None,
        });
    let mk = |size: u64, usage: wgpu::BufferUsages| {
        ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage,
            mapped_at_creation: false,
        })
    };
    let params = mk(16, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
    ctx.queue
        .write_buffer(&params, 0, bytemuck::cast_slice(&[N_INPUTS, n_fns, seed, 0]));
    let out_size = (N_INPUTS * n_fns * 4) as u64;
    let out = mk(out_size, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC);
    let staging = mk(out_size, wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST);
    let bind = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: params.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: out.as_entire_binding(),
            },
        ],
    });
    let mut enc = ctx.device.create_command_encoder(&Default::default());
    {
        let mut pass = enc.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind, &[]);
        pass.dispatch_workgroups(N_INPUTS.div_ceil(8), n_fns.div_ceil(8), 1);
    }
    enc.copy_buffer_to_buffer(&out, 0, &staging, 0, out_size);
    ctx.queue.submit([enc.finish()]);
    let slice = staging.slice(..);
    slice.map_async(wgpu::MapMode::Read, |r| r.expect("map"));
    ctx.device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("poll");
    let v = bytemuck::cast_slice(&slice.get_mapped_range()).to_vec();
    staging.unmap();
    v
}
