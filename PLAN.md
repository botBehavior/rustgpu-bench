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

## Approvals (Carter writes lines here, e.g. `approved: E1 name=rustgpu-bench 2026-06-11`)

- approved: E1 name=rustgpu-bench (Carter in chat, 2026-06-10)
- approved: E2 (Carter in chat, 2026-06-10)
- approved: E3 (Carter in chat, 2026-06-10)
- E4: NOT approved — Carter "unclear", awaiting decision

## Loop log (append one line per completed item: date, item, outcome)

- 2026-06-10 A1+A2+A3: repo initialized, initial commit 2b8f940 (27 files), tag bench-2026-06-10, README written. Phase A complete.
- 2026-06-10 B1–B5: spirv-stats tool; gap root-caused — matmul = bounds checks (unchecked: 2.5× faster, beats hand-WGSL 2.1×), tracer = codegen shape (40-Phi mega-fn vs 11 structured fns; libm + bloat hypotheses refuted), naga arm tax = wgpu re-injected checks, qptr = correct but perf-neutral. ANALYSIS.md written. Phase B complete.
- 2026-06-10 C1–C3: cold-repro PASSED end-to-end, browser steady-state 3.6 ms (~260× vs wasm), hero image. Phase C complete.
- 2026-06-10 D1–D4: all three drafts written to drafts/ (awaiting-review), Carter notified. Phase D complete. Only RED gates (E) + steady-state (F) remain.
- 2026-06-10 E1–E3: repo public (botBehavior/rustgpu-bench), Pages live, discussion #614 posted upstream. E4 awaiting Carter. F2 thread-watch now ACTIVE on #614.
- 2026-06-10 F3+G1: Phase G (Physarum) green-lit and started; kernels + tests + entries committed (c1947e3). #614: 0 replies yet.
- 2026-06-10 G2+G3: sim verified GPU-vs-CPU (1-agent bit-identical both paths); web sim live at /sim.html — 256k agents @ 12.4 ms/frame headless-verified, wasm fallback works. Only G4 (publish gate, RED) + E4 + F-watches remain.
- 2026-06-10 H+I+J phases added (Carter: "complete both" + novel-projects slate after). H1 done (c7025ca): shaderlib with 13 tests, SPIR-V-proven via demo_plasma_cs.
- 2026-06-10 H2–I3 (b952562): gallery shipped — 4 shaders CPU-oracle-gated (3e-8..2e-4), live at /gallery.html (~8 ms/frame headless-verified), source pane shows the actual library Rust. J1 slate drafted. ALL of projects #1-#5 from the original list now built. Remaining: RED gates (E4, G4, crates.io publish) + Carter's pick from novel-projects.md + F-watches.
