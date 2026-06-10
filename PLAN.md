# PLAN — ship the rust-gpu demo + benchmark properly

Loop protocol: pick the **first unchecked, unblocked** item top-to-bottom. GREEN items:
do them. YELLOW items: produce the artifact in `drafts/`, never send. RED items: skipped
until the matching line appears in the **Approvals** section at the bottom (Carter writes
it, or says it in chat). Check items off as `[x] (YYYY-MM-DD note)`. Commit after every
completed item with trailer `Co-Authored-By: Ferra <Ferra@Fable5>`. Rules of conduct:
`../docs/ETIQUETTE.md` (in rustGuru) — absolute.

## Phase A — version control foundation (GREEN)

- [x] (2026-06-10 done; repo on main, .gitignore as specced) A1. `git init` in `gpu/` (rustGuru root stays a non-repo; gpu/ is self-contained).
      `.gitignore`: `target/`, `out-*.ppm`, `shaders/spv/`, `shaders/wgsl/`,
      `web/runner_web.wasm`, `web/kernels.wgsl`, `drafts/` (drafts stay local-only).
      Keep: all sources, `Cargo.lock`, `RESULTS.md`, `bench-results.json`, `PLAN.md`.
- [x] (2026-06-10 commit 2b8f940, tag bench-2026-06-10, tests green) A2. Initial commit (whole workspace, building + tests green first). Then tag
      `bench-2026-06-10` on the commit containing today's bench-results.json.
- [x] (2026-06-10 written, included in initial commit) A3. README.md for the repo: what it is (tri-target demo + first rust-gpu vs
      hand-WGSL benchmark), results table, repro commands, gotcha list, honest caveats.
      Written for eventual public eyes, lives private until E1.

## Phase B — explain the 1.84× tracer gap (GREEN)

- [x] (2026-06-10 built tools/spirv-stats on rspirv; histograms done both arms) B1. Get a SPIR-V disassembler working (try in order: `spirv-dis` from an installed
      Vulkan SDK; `cargo install spirv-tools` if it ships bins; else a 20-line bin crate
      on `rspirv` that dumps opcode histograms). Produce opcode histograms for `render_cs`
      from (a) the rust-gpu .spv and (b) the hand-WGSL compiled to SPIR-V via `naga`.
- [x] (2026-06-10 via SPIR-V-level stats: 1 fn/74blk/40Phi vs 11 fn/84blk/~0Phi, counts in ANALYSIS.md) B2. Structural diff: transpiled-rust-gpu WGSL (`web/kernels.wgsl`) vs hand
      `shaders-wgsl/render.wgsl` — count branches, temporaries, bounds checks, loop
      shapes. Record concrete differences, not impressions.
- [x] (2026-06-10 matmul_unchecked: 2.5× CONFIRMS bounds checks; render_v2: no change, can't discriminate inlining-by-policy; transcendental test unnecessary — ExtInst evidence refutes it directly) B3. Hypothesis micro-tests (one kernel each, added to bench as optional workloads):
      (a) transcendental-heavy loop (sin/cos/sqrt torture) rust-gpu vs hand-WGSL —
      isolates math-function lowering; (b) slice-indexing-heavy loop vs the same logic
      with iterator/`get_unchecked` (if rust-gpu accepts it) — isolates bounds checks;
      (c) struct-returning function chain — isolates the Sphere/Hit struct pattern.
- [x] (2026-06-10 builds via RUSTGPU_CODEGEN_ARGS env, verifies correct on all workloads, perf unchanged — semantics project not perf lever) B4. Try the experimental qptr pipeline on the tracer
      (`RUSTGPU_CODEGEN_ARGS="--no-infer-storage-classes --spirt-passes=qptr"` — find the
      cargo-gpu way to pass it; if it builds, bench it; if not, record the failure mode).
- [x] (2026-06-10 ANALYSIS.md written: 2 refuted, 1 confirmed, 1 supported-not-isolated, naga-tax bonus finding; RESULTS/README updated) B5. Write `gpu/ANALYSIS.md`: which hypothesis(es) the data confirms/refutes, with
      numbers. Update RESULTS.md's "candidate causes" paragraph to match the evidence.
      Commit + tag `analysis-<date>`.

## Phase C — hardening for publication (GREEN)

- [x] (2026-06-10 ALL COMMANDS PASSED in fresh clone; numbers reproduce within noise) C1. Cold-repro test: clone the gpu repo to a temp dir, follow README verbatim in a
      fresh shell, confirm every command works (this catches absolute paths, missing
      steps, the cargo-gpu stub trap). Fix whatever breaks.
- [x] (2026-06-10 steady-state 3.6 ms median / first frame 197 ms @ 8spp; RESULTS updated) C2. Browser steady-state timing: extend web `?auto` mode to render N=10 GPU frames
      and report first-frame (compile-included) vs steady-state median separately.
      Re-run headless Chrome, record both numbers in RESULTS.md.
- [x] (2026-06-10 ppm2png tool, assets/render-gpu.png in README, page verified via http.server in C2 run) C3. Demo polish: PNG export of the render for the README (convert out-gpu.ppm),
      final copy pass on index.html, verify the page works from a plain
      `python -m http.server` per README.

## Phase D — publication drafts (YELLOW → drafts/, never sent)

- [x] (2026-06-10 drafts/upstream-issue.md) D1. `drafts/upstream-issue.md`: rust-gpu repo discussion/issue draft — benchmark
      methodology, results table, B-phase findings, repro link (assumes E1 done; leave
      the URL as a placeholder). One reproducible claim, peer-engineer tone, AI
      assistance disclosed.
- [x] (2026-06-10 drafts/blog-post.md) D2. `drafts/blog-post.md`: the tri-target story ("one function, four targets") +
      benchmark, honest-edges framing per ETIQUETTE.
- [x] (2026-06-10 drafts/social.md, 3 variants) D3. `drafts/social.md`: 2-3 short blurb variants linking post + repo.
- [x] (2026-06-10 summarized to Carter in chat) D4. Notify Carter: drafts ready for review (summarize in chat at next interaction;
      list what each RED gate would do).

## Phase E — RED gates (each needs an Approvals line below)

- [x] (2026-06-10 approved+done: github.com/botBehavior/rustgpu-bench public, main+tags pushed, drafts/ confirmed excluded) E1. Create public repo. [RED]
- [x] (2026-06-10 approved+done: botbehavior.github.io/rustgpu-bench live-verified, demo served from /docs) E2. GitHub Pages for the web demo from the public repo; verify live URL. [RED]
- [x] (2026-06-10 approved+done: posted to Show-and-tell as Rust-GPU/rust-gpu#614 after checking for duplicates (#315, #63 referenced) and AI policy (none); disclosure included) E3. File the upstream issue/discussion from D1 (with live links). [RED]
- [ ] E4. Publish the post (venue per Carter at approval). [RED]

## Phase F — steady state (GREEN, after E-phase items land)

- [ ] F1. Release watch: check rust-gpu releases/tags on each loop pass; on a new
      release, re-pin, rebuild, re-run bench, append a row to a tracking table in
      RESULTS.md, commit, tag.
- [ ] F2. Thread watch (only after E3): check our issue thread for maintainer replies;
      respond in-thread promptly (GREEN within an approved thread), flag anything
      substantive to Carter.
- [x] (2026-06-10 Phase G written; Carter green-lit in chat: "3") F3. Propose the Phase-2 plan for project #4 (sim toy) as a new PLAN section —
      proposal only, Carter green-lights scope.

## Phase G — the sim toy: Physarum slime mold (GREEN except G4)

Scope approved by Carter 2026-06-10. Lives in this repo/workspace (shares all plumbing;
extraction to its own repo can be decided at the G4 gate). Hundreds of thousands of
agents: sense trail → steer → move → deposit; trail diffuses + decays; render the trail.
Web page is the primary artifact; everything kernel-side is shared Rust, tested on CPU.

- [x] (2026-06-10 c1947e3: kernels + 5 tests + 3 entries, SPIR-V compiles clean) G1. `shared/src/physarum.rs`: Agent + SimParams (`#[repr(C)]`, u32/f32 only),
      agent-update kernel (sense L/C/R, steer, move, wrap, deposit) and diffuse+decay
      kernel as pure functions; unit tests (determinism, wrap, decay bounds, steering).
      v1 uses non-atomic deposits — races lose a few deposits visually, fine; determinism
      verified at single-agent level (note this honestly). `#[spirv]` entries:
      `physarum_update_cs`, `physarum_diffuse_cs` (ping-pong trail buffers).
- [x] (2026-06-10 runner-native --sim: spawn parity, 1-agent x20 bit-identical both paths, diffuse 6e-8, 10k smoke PASS) G2. GPU verify: extend a runner (or small sim-verify bin) to run 1 agent + diffuse
      N steps on GPU vs CPU, exact/statistical compare per kernel; plus a multi-agent
      smoke (total trail mass sanity). Commit with tests green.
- [x] (2026-06-10 7ca6090: headless-verified 12.4 ms/frame @ 256k agents GPU + wasm-CPU fallback; readback presentation chosen for v1; deployed to docs/) G3. Web page `web/sim.html`: WebGPU, three dispatches per frame
      (update → diffuse → present), trail presented via canvas (fullscreen pass or
      ImageData readback — pick what ships, note the choice), controls (agent count,
      speed/turn/sensor sliders, decay, pause/reset, fps + agents/sec readout), wasm-CPU
      fallback mode at small N for the same one-source story. Headless-Chrome verify
      with beacons (reuse the ?auto pattern), then copy into docs/ alongside the bench
      demo with cross-links.
- [ ] G4. Publish gate [RED]: announce/link the sim (README section + Pages index link
      are GREEN once verified; anything outward — social, posts, upstream mention —
      needs an Approvals line). Also decide: keep in rustgpu-bench or extract to its own
      repo.

## Phase H — the cargo-test-able shader library (GREEN except crates.io publish)

Original project #3, green-lit by Carter 2026-06-10 ("complete both"). Working name
`gpu-shader-lib` (final crate name + any crates.io publish = RED gate). The pitch: shader
math as an ordinary documented, unit-tested `no_std` crate — compiles to SPIR-V and runs
under `cargo test`, a DX story WGSL structurally cannot tell.

- [x] (2026-06-10 c7025ca: 13 tests green, demo_plasma_cs proves full lib compiles to SPIR-V) H1. `shaderlib/` crate: `sdf2` (circle, rounded box, segment, union/subtract/
      smooth-min), `noise` (hash, value noise, FBM with bounded octaves), `color`
      (hsv→rgb, sRGB encode/decode, Reinhard + ACES-fit tonemaps, IQ cosine palette).
      All `no_std`-compatible, SPIR-V-subset-safe (no checked math, no usize in data),
      unit tests for known values/ranges/symmetries. Workspace member; tests green.
- [x] (2026-06-10 b952562: gallery-render gates all 4 vs CPU oracle, diffs 3e-8..2e-4, PNGs in assets/gallery) H2. Prove it on GPU: 3-4 demo "fragment-style" compute entries in `shaders/`
      (uv → color via shaderlib: SDF scene, FBM clouds, palette plasma), rendered to
      PNG via the existing runner plumbing; pixel-sanity vs CPU evaluation of the same
      functions (statistical gate as with the tracer). Commit images to assets/.
- [x] (2026-06-10 rustdoc inline throughout; README "Also in this repo" section) H3. Docs: rustdoc on every public fn, README section ("the shader library"),
      note in RESULTS/ANALYSIS if any kernel hits a subset edge worth recording.

## Phase I — "Rust Shadertoy" gallery (GREEN to build/deploy; announcing = RED)

Original project #5, v1 = precompiled gallery (no server-side compile).

- [x] (2026-06-10 four launched: plasma, amoeba, clouds, mandelbrot — shaderlib::gallery with tests) I1. 4-6 launch shaders built on shaderlib (plasma, FBM terrain-shaded clouds,
      SDF scene w/ soft shadows, raymarched blobs (the amoeba!), tunnel, mandelbrot),
      each a small `#[spirv]` entry + its Rust source displayed verbatim.
- [x] (2026-06-10 live render + marker-extracted source pane + thumbnails; time uniform; wasm fallback skipped per spec) I2. `web/gallery.html`: thumbnail grid → click opens live WebGPU render with the
      Rust source side-by-side (sources shipped as text next to the page; entry picked
      per shader). Time uniform for animation. wasm-CPU fallback optional, skip if it
      drags.
- [x] (2026-06-10 headless: all four ~8 ms/frame, pixels sane; deployed to docs/ with cross-links both ways) I3. Headless verify (?auto beacons per entry), deploy to docs/ with cross-links
      from index + sim pages, README section. Announcing anywhere = RED.

## Phase J — what's next (proposal only)

- [x] (2026-06-10 drafts/novel-projects.md: 6 ideas, ranked, picks suggested — awaiting Carter) J1. After H+I ship: write a "novel projects" slate (5+ ideas that do NOT retread
      demo/benchmark/sim/library/gallery — think: things only this stack can do) into
      drafts/novel-projects.md for Carter to pick from. Proposal only.

## Phase K — differential compiler fuzzer for rust-gpu (GREEN; filing findings = RED per item)

Carter-approved 2026-06-10 ("Number two as a base"). Auto-find miscompiles/subset edges
by generating pure Rust fns, compiling CPU + SPIR-V, diffing outputs over input sweeps.

- [x] (2026-06-10 bfc13f9: e2e smoke 96 fns / 393k comparisons / 0 findings, ~7s/batch; shrinker deferred until a finding exists to test it) K1. `tools/diff-fuzz`: AST generator for small pure `fn(u32, u32) -> u32`
      functions (integer/bool ops, comparisons, if/select, bounded while, casts —
      bit-exact domain first; f32 with divergence classification is phase 2).
      Emits batches of 64 fns as a generated shader-crate module + a CPU interpreter
      of the same AST (ground-truth disagreements re-checked by compiling the
      generated source natively with rustc — rustc-CPU is final arbiter).
- [x] (2026-06-10: 500 batches / 24k fns / 98M comparisons / 2 findings; ddmin+shrink via --bisect; both minimized standalone) K2. Harness: cargo-gpu build per batch (~2-3 s), one dispatch per fn over an
      input sweep (LCG, 64k pairs), compare vs interpreter; on mismatch: shrink the
      AST (drop nodes while mismatch persists), emit minimized repro .rs + report to
      `fuzz-findings/`. Run a first campaign (≥500 batches / 32k fns); log stats.
- [x] (2026-06-10: both repros minimized (9 + 15 nodes), shared comparison-fold motif identified, drafts/upstream-miscompile-issue.md awaiting Carter [RED to file]) K3. Findings triage: each real miscompile/ICE → drafts/ as an upstream issue
      draft (RED to file); subset-edge catalog → ANALYSIS.md appendix + cheatsheet.
      Null result is also a result: "32k generated fns, no integer miscompiles" is a
      conformance statement worth posting (RED).

## Phase L — the live-codable audio-visual synth (BLOCKED until Phase K complete)

Carter-approved 2026-06-10 ("I love 6 & 4 together as an audio visual synth"); Carter
directive same day: **own repo, started only after Phase K is complete.** Projects
#6 + #4 fused: GPU kernels synthesize audio AND visuals from one patch state; the
live-coding loop (2-s rebuild, hot-swap) makes it a playable instrument whose sound
and look are Rust source.

- [x] (2026-06-10 github.com/botBehavior/oscilla PUBLIC, day-zero scaffold: synth crate, sine+saw with zero-crossing pitch tests, 3 green; local C:\Users\carte\projects\oscilla) L0. New repo `botBehavior/<name>` (Carter picks name at the Approvals line —
      suggestions: `oscilla`, `rust-av-synth`, `ferrosynth`, `wavesmith`). Scaffold
      from rustgpu-bench plumbing (cargo-gpu build, verify-gate pattern, headless
      harness); depends on gpu-shader-lib via git (or path until published).
      [RED: repo creation needs the Approvals line with the chosen name]

- [x] (2026-06-10 oscilla 5a4be41: FM/saw/sine + closed-form ADSR + one-pole + mixer, 5 test suites green; lives in oscilla-synth not shaderlib — own repo per Carter; gallery-primitive visuals deferred to the page milestone) L1. `shaderlib::synth`: oscillators (sine/saw/pulse/FM pair), ADSR envelope,
      one-pole filter, voice mixer — pure `fn(sample_index, params) -> f32` block
      renderer + a visual kernel fed by the same patch state (level/pitch reactive,
      built on gallery primitives). CPU tests: pitch via zero-crossing count,
      envelope monotonicity, output bounded [-1, 1].
- [x] (2026-06-10 oscilla: 8 chained GPU blocks within 1.46e-5 of CPU oracle, boundaries continuous, visual 2.4e-7; two new subset edges documented: subslicing + array-unsizing don't lower) L2. GPU verify + entries: `synth_audio_cs` (one thread per sample, block-sized
      dispatch), `synth_visual_cs`; runner verification vs CPU rendering of the same
      block (bit-tolerant float gate as established).
- [x] (2026-06-10 oscilla 8e66fb4: playable page with worklet ring + honest margin readout; headless rms 0.259/peak 0.459; known perf item = batch blocks per dispatch) L3. `web/synth.html`: QWERTY keyboard → patch state; GPU renders audio blocks
      ahead of the play cursor → readback → AudioWorklet ring buffer; visual kernel
      renders to canvas in the same frame loop; status shows block budget vs deadline
      (the honest metric: GPU audio is about meeting realtime, report it truthfully).
      Headless verify: block waveform statistical-gate vs CPU + visual pixel sanity
      (audio output itself can't be heard headless — note that honestly).
- [ ] L4. Live-coding mode (the #4 half): `tools/live.ps1` — watch `shaderlib/src/`,
      on change: cargo-gpu build → naga → kernels.wgsl; page polls the file's
      modified-stamp and hot-swaps pipelines WITHOUT stopping the audio ring.
      Measure and report edit→sound latency. Local-dev experience; documented in
      README with a recorded demo as a future E-gate asset.
- [ ] L5. Deploy static synth to oscilla's own Pages site (cross-link from
      rustgpu-bench pages); announcing = RED.
- [ ] L6. BOLD scope (Carter: "make it a bold project") — after L1-L5 land, in order:
      polyphony (8+ voices, voice stealing); WebMIDI input (play it with real keys);
      patch presets + shareable patch URLs (state in the fragment); record-to-WAV
      export; spectrum + oscilloscope visual modes built from the audio block (the
      "oscilla" namesake). Each lands as its own verified increment, same discipline.

## Approvals (Carter writes lines here, e.g. `approved: E1 name=rustgpu-bench 2026-06-11`)

- approved: L0 name=oscilla (Carter in chat 2026-06-10: "oscilla is a bold name, let's make it a bold project") — repo botBehavior/oscilla, create public when Phase K is complete
- approved: E1 name=rustgpu-bench (Carter in chat, 2026-06-10)
- approved: E2 (Carter in chat, 2026-06-10)
- approved: E3 (Carter in chat, 2026-06-10)
- E4: NOT approved — Carter "unclear", awaiting decision

## Loop log (append one line per completed item: date, item, outcome)

- 2026-06-10 PHASE K COMPLETE (9864019): 2 standalone minimized miscompiles (9 + 15 nodes, shared comparison-fold motif: 0<x*EVEN_C and y<(C|y) selects), upstream draft awaiting Carter. L0 DONE: botBehavior/oscilla public, scaffold pushed, 3 tests green. L1 next.

- 2026-06-10 A1+A2+A3: repo initialized, initial commit 2b8f940 (27 files), tag bench-2026-06-10, README written. Phase A complete.
- 2026-06-10 B1–B5: spirv-stats tool; gap root-caused — matmul = bounds checks (unchecked: 2.5× faster, beats hand-WGSL 2.1×), tracer = codegen shape (40-Phi mega-fn vs 11 structured fns; libm + bloat hypotheses refuted), naga arm tax = wgpu re-injected checks, qptr = correct but perf-neutral. ANALYSIS.md written. Phase B complete.
- 2026-06-10 C1–C3: cold-repro PASSED end-to-end, browser steady-state 3.6 ms (~260× vs wasm), hero image. Phase C complete.
- 2026-06-10 D1–D4: all three drafts written to drafts/ (awaiting-review), Carter notified. Phase D complete. Only RED gates (E) + steady-state (F) remain.
- 2026-06-10 E1–E3: repo public (botBehavior/rustgpu-bench), Pages live, discussion #614 posted upstream. E4 awaiting Carter. F2 thread-watch now ACTIVE on #614.
- 2026-06-10 F3+G1: Phase G (Physarum) green-lit and started; kernels + tests + entries committed (c1947e3). #614: 0 replies yet.
- 2026-06-10 G2+G3: sim verified GPU-vs-CPU (1-agent bit-identical both paths); web sim live at /sim.html — 256k agents @ 12.4 ms/frame headless-verified, wasm fallback works. Only G4 (publish gate, RED) + E4 + F-watches remain.
- 2026-06-10 H+I+J phases added (Carter: "complete both" + novel-projects slate after). H1 done (c7025ca): shaderlib with 13 tests, SPIR-V-proven via demo_plasma_cs.
- 2026-06-10 K1 (bfc13f9): fuzzer e2e; K2 campaign (500 batches, seeds 100..600) launched in background — may need chunked restarts if the 10-min task cap kills it; continue from last reported seed.
- 2026-06-10 K3 progress: shrinker v1 built (batched 48-candidate probes); initial "context-dependent" conclusion was WRONG — an artifact of batched probing (v1's final check had also dropped the loop). CORRECTED by --bisect: ddmin removed ALL 47 siblings, repro is STANDALONE; minimized to a 15-node single fn: 4-iter loop of acc = acc*1664525 + (y + select(y < (C|y), x, select(x < C2, x, y))). The loop + select combination is essential. findings/min-module-seed303.rs committed. Probe budget hit at 80; optional further shrink + seed423 bisect + upstream draft remain.
- 2026-06-10 K2 CAMPAIGN COMPLETE: 500 batches / 24,000 fns / ~98M comparisons / 2 findings (seed303-fn44, seed423-fn43 — both bounded-loop + masked-shift patterns). TRIAGED: both reproduce with IDENTICAL wrong values through the naga frontend → two independent SPIR-V consumers agree with each other and disagree with native rustc → **rust-gpu codegen miscompile confirmed**, not a driver bug. Repros preserved in findings/ (committed). Next: implement shrinker (--shrink mode), minimize both, then upstream issue draft (RED).
- 2026-06-10 #614 now 9 comments: Firestar99 ran 2048²×32spp throughput — RADV at scale: rust-gpu v1 25.2ms vs hand 18.4ms (1.37×), **render_v2 18.2ms = PARITY** (by-value variant matters at scale on RADV!); nazar-pc's check question answered in-thread. Reply draft needs rewrite (v2 finding + current trusted-module API name); still permission-blocked for posting.
- 2026-06-10 K2 FIRST FINDING at ~12k fns: mismatch-seed303-fn44 (nested masked shifts in bounded loop) — native rustc CONFIRMS interpreter, GPU result wrong. Triage pending (K3): shrink + naga-path + qptr cross-tests to split rust-gpu codegen vs NVIDIA driver. Do NOT report upstream until shrunk + triaged (drafts/ + RED).
- 2026-06-10 #614: Firestar99 replied with AMD/RADV run — TRACER GAP IS DRIVER-SPECIFIC (parity on RADV); glam trap already fixed upstream (#613); cargo-gpu stub workaround: cargo install cargo-gpu@0.10.0-alpha.1; nazar-pc asked re: check escape (answer: create_shader_module_trusted + ShaderRuntimeChecks::unchecked). Reply drafted (drafts/614-reply-1.md) but PERMISSION-BLOCKED by harness classifier — needs Carter to post or grant permission. ANALYSIS.md updated with attribution.
- 2026-06-10 H2–I3 (b952562): gallery shipped — 4 shaders CPU-oracle-gated (3e-8..2e-4), live at /gallery.html (~8 ms/frame headless-verified), source pane shows the actual library Rust. J1 slate drafted. ALL of projects #1-#5 from the original list now built. Remaining: RED gates (E4, G4, crates.io publish) + Carter's pick from novel-projects.md + F-watches.
