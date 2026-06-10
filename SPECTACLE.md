# Mycelia — the visual-spectacle proof point

**Chosen 2026-06-10** (Carter: "Massive visual spectacle") as the web-supremacy proof point,
after the rust-gpu compiler-contribution path was retired. Max-effort build.

## The thesis (honest framing)

Not "JS can't render this" — WebGPU underlies both. The real, defensible claim:

> **A living, million-agent multi-species simulation whose entire kernel is ordinary,
> unit-tested Rust — running on your GPU via rust-gpu, and verifiable bit-identical on the
> CPU.** Gorgeous, interactive, shareable.

The differentiator is Rust's correctness + testability + one-codebase-CPU-and-GPU meeting
WebGPU's power. The spectacle makes that visceral. Lead with the experience; the Rust story
is the substance underneath.

## What it is

Multi-species Physarum: several agent colonies sharing a high-res trail field. Each senses
its OWN channel (attraction) and is REPELLED by the others, carving competing territories
with luminous boundaries. Millions of agents, 60 fps, palette + bloom rendering, mouse
interaction, presets. The boundaries between colonies are where the beauty lives.

## Plan

- [x] **S1. Kernel** (`shared/src/mycelia.rs`, 6 CPU tests green). Multi-species
      (`N_SPECIES=3`, planar trail channels), Jones agent rule generalized to signed
      (repulsive) signals, mouse force in-kernel, deterministic spawn/update/diffuse. The
      tested-Rust foundation = the proof.
- [ ] **S2. GPU entries** (`shaders/src/lib.rs`): `mycelia_spawn_cs` (init agent buffer),
      `mycelia_update_cs` (one thread/agent: update_agent → non-atomic deposit into own
      channel of the trail-write buffer), `mycelia_diffuse_cs` (one thread/cell/channel:
      diffuse_at, ping-pong), `mycelia_render_cs` OR a fragment pass: trail channels → RGB →
      IQ palette + bloom + tonemap (reuse `gpu-shader-lib::color`). Verify a single-agent +
      diffuse step GPU-vs-CPU (bit-exact diffuse; agents statistical) like the existing sim.
- [ ] **S3. Web page** (`web/mycelia.html`): full-bleed canvas at devicePixelRatio; scale
      agents to whatever holds 60 fps on the 5070 Ti (target millions); per-frame passes
      update → diffuse(×k) → render; mouse → params (attract/repel); a tasteful control set
      (species count, sensor angle/dist, speed, decay, self/other weights, palette) and
      morphing PRESETS; FPS + agent-count readout; screenshot/share. Gorgeous first, knobs
      second. wasm-CPU fallback at small N for the same-code story.
- [ ] **S4. Polish + verify + deploy**: headless-Chrome `?auto` beacons (frame renders,
      reactivity, agent count); tune the look until it's genuinely mesmerizing (Carter's
      eyes are the gate); deploy to `docs/`, cross-link from index. README section with the
      honest thesis + a hero capture.
- [ ] **S5. [RED] Package + announce**: decide own-repo extraction (Carter picks a name —
      "mycelia" is the working name) + Pages site; any outward announce needs an Approvals
      line.

## Reuse
- Kernel pattern + GPU-verify + headless harness: the existing Physarum sim
  (`shared/src/physarum.rs`, `web/sim.html`, `shaders/` entries `physarum_*`).
- Palettes/tonemap/color: `gpu-shader-lib` (git dep, already a cross-repo consumer via oscilla).
- Rendering reference: the gallery (`web/gallery.html`) fullscreen-fragment pattern.

## Honest risks
- "Another physarum demo" — beat it with scale (millions), multi-species boundaries, and
  rendering polish. The look has to be exceptional, not just functional.
- Non-atomic deposits race at scale (documented; visually fine). If banding appears, switch
  the deposit to u32 atomic fixed-point.
- Carter's gate is his eyes — budget for look iteration, not just "it runs."
