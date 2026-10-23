---
id: AUD-044
epic: E02
feature: F42
title: Audit F42
status: done
points: 1
---

# E02 · F42 · AUD-044 — Audit F42

Audit F42 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F42's criteria, when the audit ends, then each is traced to a test titled `[F42.ACn] …`, broken once on purpose and seen failing.

## Tasks

- [x] T1 — Criteria traced, findings fixed (1 h)

One task = one commit, referenced as `AUD-044.T1`.

## Findings

The `auditor` alone reviewed F42 read-only (ADR-0086): no error in the mathematics, both derivations holding and every expected value recomputed matching; each finding below reproduced before it was fixed. F42's five criteria are traced to the `[F42.AC1]` tests of `src/model/view/` (`pointUnderPointer`, `zoomedView`, `wheelZoomFactor`, `wheelPixels`) and `zoom-and-pan.test.ts`, the `[F42.AC2]` tests of `panned-view.test.ts` and `zoom-and-pan.test.ts`, the `[F42.AC3]` tests of `stepped-scale.test.ts`, `fitted-view.test.ts`, `zoom-keys.test.ts` and `zoom-and-pan.test.ts`, the `[F42.AC4]` tests of `zoom-and-pan.test.ts` and `view-helps.test.ts`, and the `[F42.AC5]` tests of `contour-band-within.test.ts` and `helps-frame.test.ts`, each broken below.

| Id | Finding | Result |
| --- | --- | --- |
| 1 | The pointer's offset went stale during a pan drag: the drag's moves are kept from the other listeners, the hover's among them, so `HOVER_OFFSETS` kept the press's offset and `+` then zoomed about it | reproduced: over (440, 340) at 200 %, dragged with Space to (540, 390), `+` gave `194.43651 154.79185 …` where (400, 300) under the pointer asks `209.08117 162.11418 424.26407 282.84271`. Fixed: the map moved to `playground/view/` as `POINTER_OFFSETS`, written by the hover and by the pan drag's moves; test `[F42.AC3] zooms by + about where a pan drag left the pointer` |
| 2 | The toolbar's History group was a `<p>` holding a `<details>`: the parser closed it there, the steps, the memory and the size left outside the group (F41) | reproduced in happy-dom: `#history-steps-box`'s group not the History one. Fixed: a `<div role="group">`; test: the steps, the memory and the size each within the group labelled History (`history-list.test.ts`) |
| 3 | "R fitted at 0.0072" cannot happen: min(1 120 / 99 999, 720 / 100 000) = 0.0072 is under the bound 0.01, so `fittedView` gives 0.01, origin (−10 000.5, 10 000) | fixed at s = 0.01: chords of 200, ⌈78 539.03 / 200⌉ = 393 a corner, each 2 · 49 999.5 · sin(π / 1 572) = 199.84 units, 1.998 pixels; the helps' box (−70 000.5, −30 000) to (169 999.5, 130 000) still holds R. `band-within-box.md` checks, `interaction.md` §2, EN-149's criterion, note 0031, `contour-band-within.test.ts`, `helps-frame.test.ts`, `piece-step-count.test.ts` |
| 4 | AC4's export had no `[F42.AC4]` test; AC5 says "redraws in under 16 ms" where its test bounds 60 redraws by 960 ms, a mean | the export test retitled `[F42.AC4] exports the same file at 100 % and at 200 % …`; "on average" in AC5 is for the main thread, the feature file being its own |
| 5 | The derivation's boundary row, a pan of 600 kept and 601 computed again, was tested at 100 and 700 only | test `keeps the helps at a pan of 600 pixels, computes them again at 601`; `isBoxWithin`'s `≤` turned `<` is caught by nothing, equivalent, its ε tolerance holding the edge; the padding 0.5 turned 0.499 or 0.501 fails it |
| 6 | `band-within-box.md` step 3 dropped and kept subpaths without stating that the band paints their union | fixed: the premise written, every subpath turning positively (`circleArcs`, `segmentBand`), so that under the nonzero rule none cancels another (`REF-SVG2-FILL-RULE`, its row's use added) |
| 7 | `view-zoom.md`, `REF-PENPOT-VIEWPORT-ACTIONS` and F42's guide said the wheel zooms by "the deltas' sum", "as Penpot"; the code takes the vertical delta only, as `interaction.md` §7 says | reproduced in Penpot's source (`on-mouse-wheel`, `schedule-zoom!`): it adds each event's horizontal delta to its vertical one, and multiplies the factors of a frame where F42 sums the deltas before one factor: two of −50 give × 1.57 here, × 1.65122 in Penpot. Fixed in the docs as the agent's choice (US-078): `view-zoom.md` step 5, `interaction.md` §7, Q60, the reference's row, `wheelZoomFactor`'s TSDoc, F42's guide |
| 8 | `REF-PENPOT-SHORTCUTS` did not hold F42's zoom keys; `REF-SVG2-COORDS` was verified before F42 used its `viewBox` | both read again on 2026-10-23: the Penpot row holds the Zoom tables (§12 and §20), `Shift` + `0`, `1`, `2`, `+`, `-`, its use F42's; the SVG row quotes §8.6 on `viewBox`, `[verified] 2026-10-23` |
| 9 | ADR-0049: pixels to user units computed inline, `scale(v, 1 / s)`, in `pointUnderPointer`, `viewOrigin` and `pannedView` | fixed: `pointUnderPointer` calls `screenToUser` per coordinate; `viewOrigin` is the point under the opposite offset of a view at that point, `pannedView`'s origin the point under the delta, both through `pointUnderPointer` |
| 10 | The uniform fit's scale, min(room w / span w, room h / span h), written twice: `fitInBox` and `fittedView` | fixed: `meetScale` (`src/geometry/layout-box/`, `REF-SVG2-COORDS` §8.2 step 7), a span of 0 bounding no scale, 0 × 0 giving `Infinity`; examples and a property; `fitInBox`'s operations unchanged, so its floats are |
| 11 | `shapeAtPoint(areas, point, screenToUser(PICK_TOLERANCE, readView(document).scale))` written three times | fixed: `readShapeAt` in `playground/hover/`, used by the hover's outline, the click's selection and the press's drag |
| 12 | `listenToZoomKeys` sat in `selection-keys/` | moved to `playground/zoom/`, no cycle: `selection-keys` no longer imports `zoom` |
| 13 | US-079's T1 and T2 were left unticked | ticked |
| 14 | A Space released while the window is unfocused never reaches the page, which then pans at every press (the auditor's question; the main thread's decision: let it go) | reproduced: Space down, the window's `blur`, then a drag panned to `130 105 …`. Fixed: `listenToSpace` drops it at the window's `blur`; test `[F42.AC2] drops Space held as the window loses the focus`; `interaction.md` §7, Q60, F42's guide |
| 15 | A pan drag's press reached no test of its isolation: the press let through to the drawing's listeners passed every test | found by a hand break (below); test `[F42.AC2] pans by a drag with Space held from A, A neither selected nor moved`, the pointer moved on after the release, which a selection drag left pending would follow |
| 16 | A percentage typed between 0 and 1 was never tested: `typed <= 0` turned `typed <= 1` survived | test: 0.5 typed gives 1 %, the bound, about the canvas's centre (`zoom-keys.test.ts`) |

Mutations (A4), `npm run mutate`: the 20 modules of `src/` F42 added or changed with a body, against their related tests, 57 caught and 1 survivor, equivalent: `steppedScale`'s type `-1 \| 1` written `-1 \| 2`, erased at run time. The 15 modules of the playground F42 added or changed with a body, against the tests of `playground/view/`, `zoom/`, `helps-frame/` and `hover/`, 30 caught and 3 survivors: `readSteppedView`'s type `-1 \| 1`, equivalent; `NO_FRAME`'s box, never reused since its scale 0 is no view's, equivalent; the typed percentage's floor, finding 16. By hand, each criterion once: `zoomedView` placing the origin at the old scale (`[F42.AC1]`, 22 fail, `[F42.AC1] zooms to 200 % about the pointer` among them); `pannedView` panning by the opposite delta (`[F42.AC2]`, 6 fail); `Shift` + `1` read as the selection's fit (`[F42.AC3] fits A, B and C by Shift + 1`, 1 fails); the export framed by the page's view (`[F42.AC4] exports the same file`, 1 fails); `contourBandWithin` testing every chord without cutting the runs (`[F42.AC5] draws R's band empty at 100 %, 60 redraws …`, 12.3 s for 960 ms) and keeping every chord (8 fail, the three `[F42.AC5]` among them). And the new tests: each seen failing before its fix (findings 1, 2, 14), `meetScale`'s against its placeholder (6), the boundary's against the padding at 0.499 and 0.501 (5 each), the press's isolation against its `stopImmediatePropagation` dropped (1), the typed floor against `typed <= 1` (1).

Properties (ADR-0022): F42's `geometry` and `math` functions had theirs; `meetScale` adds its own, the span within the room on both axes and on its side on one.

Breadth (step 7): `check:all` passes with the push; `AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, `docs/conventions/` and `docs/tooling/` still describe what F42 did; `exceptions.md` gains no row.

Next feature (G): F42 closes E02; the next feature is the main thread's to choose. Left for later review, not fixed here: Safari's pinch (`GestureEvent`), a fit holding the stroke, `Shift` + `Ctrl` + wheel on macOS sending `deltaX`, `Space` on a focused toolbar button or the zoom's summary.

Evolvability (H):

- Size: `src/model/view/` 15 modules, `playground/view/` 14 with `pointer-offsets`, `playground/zoom/` 8 with `listenToZoomKeys`, `playground/hover/` 15 with `readShapeAt`.
- Tools: `npm run mutate` copies the tracked files only: a module not yet staged is "not a tracked text file", and a barrel exporting one makes every related test fail before any mutation; staging the work first was enough.
- Fitness functions: AC5's 16 ms is a mean of 60 redraws (finding 4).
- Patterns: a listener that keeps events from the others also keeps from them what they record (finding 1); a shared record belongs to the folder that writes it first.
- Agent configuration: nothing.
- Process: a value claimed for a bounded quantity is computed through its bound (finding 3).

## For the main thread

- For the done commit's footer: the rule docs changed in this task, `interaction.md` §2 (R fitted at 1 %) and §7 (the wheel's vertical deltas summed, Space let go at the window's blur), the Q row Q60 of `docs/domain/README.md`, and F42's guide page; F42 has no `VAL` (VAL-043 dropped).
- AC5 of F42's feature file says "redraws in under 16 ms", checked as 60 redraws within 960 ms: "on average" to add to it (finding 4), the feature file being the main thread's.
- Penpot multiplies a frame's zoom factors where F42 sums the deltas before one factor (finding 7): two turns of −50 zoom by 1.57 here, 1.65122 in Penpot. Written as the agent's choice; following Penpot instead would change `WHEEL_ZOOMS` to keep a factor, and the test of two −50 to 165 %.
- Penpot's workspace zooms with `⌘` `⇧` `0`, `1` and `2` on macOS (`REF-PENPOT-SHORTCUTS`, §12), where F42 leaves every key with `Cmd` to the browser (SP-030's answer 3): for the review, as the other points left.
