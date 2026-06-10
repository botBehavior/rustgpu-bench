# Mycelia — an adaptive mycelial ecosystem on the GPU (master plan)

**Chosen 2026-06-10.** The flagship web-supremacy proof point, in its real form: not a
slime-mold screensaver but a **living fungal organism that visibly solves its environment** —
grows toward food, pulls resource home through reinforcing cords, prunes dead ends, clashes
into spalted-wood zone lines, and (later) fruits and reseeds. Engine = adaptive transport
(Tero et al., *Science* 2010, adaptive network design); body = foraging + zone lines; payoff =
the fruiting life cycle.

## Thesis (locked)

> A living mycelial ecosystem — hyphal growth, resource transport, adaptive network
> optimization, competition, and reproduction — whose entire simulation is ordinary,
> **unit-tested Rust**, compiled to your GPU by rust-gpu and verifiable on the CPU.

The honest boundary-push: **a coupled per-frame relaxation solver with adaptive feedback**,
in rust-gpu, is something no rust-gpu demo has shown — it proves the stack does serious
scientific simulation, not toys. The "it optimizes the network" behavior is a *citable*
result, not hand-waving.

## The model (biology → simulation)

### Fields (planar f32 buffers at sim resolution `W×H`)
- `nutrient` — substrate food. Consumed by foraging; static patches (later: slow regrow).
- `biomass[c]` per colony `c` — the permanent hyphal network. Doubles as **conductivity**:
  resource flows only where biomass exists. This is what glows.
- `resource[c]` per colony — transportable sugars. The thing the solver moves.
- `flux[c]` per colony — scratch: resource throughput per cell this frame (drives adaptation).
- (T5+) `barrier` — zone lines. (T6+) `moisture`, `fruiting` markers.

### Agents — hyphal tips (NOT a swarm; tens of thousands of *tips*)
`Tip { x, y, heading, colony, alive }`. Tips extend the network; the population grows by
branching and shrinks by anastomosis/atrophy. v1: fixed-max array + `alive` flag + atomic
spawn counter; upgrade to indirect-dispatch compaction only if needed.

### The transport solver — THE HEART (Tero adaptive model, field form)
Per relaxation iteration, per colony (Jacobi):
- conductivity between cell `i` and 4-neighbor `j`:  `k_ij = min(B[i], B[j])`  (cords = sharp).
- resource diffuses, weighted by conductivity:
  `R'[i] = R[i] + α · Σ_j k_ij · (R[j] − R[i])`,  α small for stability
  (`α · 4 · max_k < 1`; clamp).
- accumulate throughput:  `F[i] += Σ_j k_ij · |R[j] − R[i]|`.
Run `K` iterations/frame (K≈4–12) so resource propagates several cells.

**Sources & sinks** (set before/after the iterations):
- foraging tip on `nutrient`:  `R[cell] += income`,  `nutrient[cell] −= forage_rate`  (source).
- growing tip:  consumes `R[cell] −= growth_cost`  (sink) — growth is **resource-limited**.

**Adaptation** (once/frame, after transport) — the feedback that makes it *intelligent*:
`B[i] += adapt_rate · F[i] − atrophy · B[i]`  (clamp ≥ 0).
High-throughput routes **thicken into cords**; unused biomass **atrophies and prunes**. This
is exactly the Tero rule and it is what makes the network find efficient supply routes.

### Tip growth (chemotropic, resource-limited)
Each frame, per tip: sense `nutrient` gradient (3 sensors) → steer up-gradient; mild avoidance
of dense own-biomass (no backtracking) and rival biomass (T5 zone lines). If `R[cell] ≥
growth_cost`: advance, deposit `B[cell] += deposit`, pay the cost; else stall. At a nutrient
cell: forage (income + deplete). Branch stochastically when resource-rich. Anastomose
(terminate, free slot) when entering established own-biomass → closed loops.

### Per-frame pass order
1. **grow** tips (sense/steer/extend/deposit/forage/branch/anastomose) → sets R sources/sinks
2. **transport** ×K (relaxation + flux accumulation)
3. **adapt** (biomass thicken/prune from flux)
4. **compete** (zone lines) — T5
5. **fruit/sporulate** — T6
6. **render** (compose fields → bioluminescent color)

## Verification strategy
- **Per-function CPU unit tests**: growth step, conductivity, one transport iteration
  (mass-conserving up to source/sink), adaptation monotonicity.
- **The killer integration test (T3, CPU): adaptive shortest path.** Seed biomass connecting a
  source and a sink by TWO routes (short + long). Run grow-free transport+adaptation N steps.
  Assert the **short route's biomass ends thicker than the long route's** — the network
  *chooses* the efficient path. This both validates the solver and demonstrates the science.
- **GPU-vs-CPU**: deterministic single-tip growth bit-comparable; the transport field
  bit/ε-comparable for a fixed biomass+resource setup (the established statistical gate).
- **Headless browser**: renders, field is alive, mouse-drop foraging visibly connects.

## Build phases

- [x] **T1 — Organism foundation.** `shared/src/mycelium.rs`: Tip (6×f32), single-colony
      chemotropic growth (Jones steer toward nutrient), permanent biomass deposit, foraging
      depletion, branching (deterministic, returns child via `Step`), anastomosis (fuse on
      thick biomass ahead), radial spore inoculation, tested `shade()` (foxfire glow). 6 CPU
      tests green. GPU entries `mycelium_spawn/grow/render_cs` compile to SPIR-V (Step struct
      lowers fine). Branch-append (atomic) deferred to T4 per note.
- [x] **T2 — The transport solver.** `resource` + `flux` fields; saturating biomass
      conductivity `k = m/(m+k_half)` (`m = min(B_i,B_j)`) that keeps the Jacobi relaxation
      stable (α ≤ ¼) and mass-conserving however thick cords grow; `transport_at` (per-cell,
      toroidal, returns new R + |throughput|). Growth is now **resource-limited**: tips forage
      nutrient→sugar at `home`, pay `growth_cost` to advance, and **stall** (hold position,
      keep foraging) when the network can't supply them. 6 new CPU tests — R conservation +
      maximum principle, flow-along-cord-not-through-gaps, frozen-without-biomass,
      flux-tracks-gradient, resource-gated growth, forage-funded growth — 12/12 green.
      `mycelium_transport_cs` + the updated `mycelium_grow_cs` compile to SPIR-V **and
      transpile through naga to WGSL** (web path verified, "double-check trap" cleared).
      2026-06-10.
- [ ] **T3 — Adaptive feedback + the shortest-path proof.** The `B += adapt·F − atrophy·B`
      rule; cords form, dead ends prune. Ship the two-route shortest-path integration test
      (the citable result). Tune stability (α, K, rates) on CPU.
- [ ] **T4 — GPU + page.** rust-gpu entries for grow/transport/adapt/render; WebGPU multi-pass
      pipeline (ping-pong resource, K transport sub-dispatches); bioluminescent render
      (biomass glow colony-hued, nutrient substrate warmth, resource glow along cords);
      mouse = drop a nutrient bolus and watch the network forage to it. Headless verify.
      GPU-vs-CPU gate on a deterministic setup.
- [ ] **T5 — Competition & zone lines.** Multi-colony: rival-biomass avoidance in growth,
      `barrier` deposition + mutual suppression at interfaces → permanent spalted-wood lines.
- [ ] **T6 — Life cycle (the payoff).** Fruiting detection (biomass+resource > threshold),
      primordium growth, spore emission (new tips at a dispersal radius) → generational
      turnover. Atomic spawn / indirect dispatch as needed.
- [ ] **T7 — Polish.** Look tuning (Carter's eye is the gate), presets (forest floor / petri
      dish / log), performance (field res × K for 60 fps), HUD, capture, README with the
      honest thesis + hero capture. Headless `?auto` beacons throughout.
- [ ] **T8 — [RED] Package + announce.** Own-repo extraction + Pages; name decision (working
      "Mycelia"; *Foxfire* is a candidate if we lean into the bioluminescence). Any outward
      announce needs an Approvals line.

## Risks & unknowns
- **Solver stability** — relaxation diverges if α/K wrong; the CPU tests + clamps catch it.
  Budget T3 tuning time.
- **Performance** — K transport iterations × W×H × C colonies per frame is the cost center.
  Lever: lower sim res than display res (render upscales), cap K, fewer colonies. Measure early.
- **GPU tip population** — branching/death needs compaction or fixed-max+flags. Start
  fixed-max+`alive`; only build indirect-dispatch compaction if the cap bites.
- **Determinism for GPU-vs-CPU** — keep growth deterministic from (tip-id, frame); transport
  is deterministic; verify on a single tip + a fixed field.
- **The look** — bioluminescent cords on dark substrate; needs render tuning and his eye (T7).

## Reuse / supersession
- The multi-species-physarum prototype (`shared/src/mycelia.rs`, `web/mycelia.html`) was the
  plumbing rehearsal — it taught us the WebGPU multi-pass + present-blit + headless-shot loop,
  and `shade()`/the present pass carry over. It becomes a simpler sibling or is retired once
  the ecosystem engine lands; not the flagship.
- Palettes/tonemap: `gpu-shader-lib`. Present-blit + `?auto` screenshot harness: from mycelia.html.

## Open decisions (defaults chosen; Carter can redirect)
- 2D top-down field (3D is a separate, much larger project). **Default: 2D.**
- Single colony through T4, multi-colony competition at T5. **Default: single-first.**
- Lives in the rustgpu-bench workspace during the build; own repo at T8. **Default: yes.**
- Name: working **Mycelia**; *Foxfire* if we center the glow. Decide at T8.
