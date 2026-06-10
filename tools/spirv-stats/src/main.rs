//! Per-entry-point opcode statistics for a SPIR-V module.
//! Usage: spirv-stats <module.spv> [entry-name]

use std::collections::HashMap;

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().expect("usage: spirv-stats <module.spv> [entry]");
    let only = args.next();
    let bytes = std::fs::read(&path).expect("read");
    let module = rspirv::dr::load_bytes(&bytes).expect("parse SPIR-V");

    // entry point name -> function id
    let entries: HashMap<u32, String> = module
        .entry_points
        .iter()
        .map(|inst| {
            let id = match inst.operands[1] {
                rspirv::dr::Operand::IdRef(id) => id,
                _ => unreachable!(),
            };
            let name = match &inst.operands[2] {
                rspirv::dr::Operand::LiteralString(s) => s.clone(),
                _ => unreachable!(),
            };
            (id, name)
        })
        .collect();

    let module_total: usize = module
        .functions
        .iter()
        .flat_map(|f| &f.blocks)
        .map(|b| b.instructions.len())
        .sum();
    let module_blocks: usize = module.functions.iter().map(|f| f.blocks.len()).sum();
    println!(
        "module: {} ({} bytes, {} functions, {} instructions, {} blocks total)",
        path,
        bytes.len(),
        module.functions.len(),
        module_total,
        module_blocks
    );
    for f in &module.functions {
        let id = f.def_id().unwrap();
        let name = entries.get(&id).cloned().unwrap_or_else(|| format!("fn%{id}"));
        if let Some(filter) = &only {
            if &name != filter {
                continue;
            }
        }
        let mut hist: HashMap<&'static str, usize> = HashMap::new();
        let mut total = 0usize;
        let mut blocks = 0usize;
        for block in &f.blocks {
            blocks += 1;
            for inst in &block.instructions {
                *hist.entry(opname(inst.class.opcode)).or_default() += 1;
                total += 1;
            }
        }
        let mut sorted: Vec<_> = hist.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        println!("\n== {name}: {total} instructions, {blocks} blocks ==");
        for (op, n) in sorted.iter().take(25) {
            println!("  {n:6}  {op}");
        }
    }
}

fn opname(op: rspirv::spirv::Op) -> &'static str {
    // Debug formatting of the enum leaks; cheap static-ish mapping via Box::leak
    Box::leak(format!("{op:?}").into_boxed_str())
}
