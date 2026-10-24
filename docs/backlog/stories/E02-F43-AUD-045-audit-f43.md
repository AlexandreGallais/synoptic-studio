---
id: AUD-045
epic: E02
feature: F43
title: Audit F43
status: ready
points: 1
---

# E02 · F43 · AUD-045 — Audit F43

Audit F43 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F43's criteria, when the audit ends, then each is traced to a test titled `[F43.ACn] …`, broken once on purpose and seen failing.

## Tasks

- [ ] T1 — Criteria traced, findings fixed (1 h)

One task = one commit, referenced as `AUD-045.T1`.

## Findings

The `auditor` alone reviewed F43 read-only (ADR-0086); each finding below reproduced before it was fixed, the Product Owner's answers of 2026-10-24 settling findings 2 and 3. F43's four criteria are traced to the `[F43.AC1]` tests of `remove-listeners.test.ts` and `select-element.test.ts`, its two times measured (finding 11); the `[F43.AC2]` tests of `helps-on-any-color.test.ts`, `shapes-taken-shown.test.ts`, `no-page-text-selected.test.ts` and `read-box-band-path-data.test.ts`; the `[F43.AC3]` tests of `write-several-values.test.ts`, `arrow-bursts.test.ts`, `held-arrows.test.ts`, `held-nudge-shift.test.ts`, `field-arrow-step.test.ts` and `is-browser-key.test.ts`; and the `[F43.AC4]` tests of `drag-shape.test.ts`, `zoom-and-pan.test.ts`, `write-several-values.test.ts` and `schedule-redraw.test.ts`, each broken below.

| Id | Finding | Result |
| --- | --- | --- |
| 1 | On macOS an arrow pressed with Cmd sends no `keyup`: `listenToHeldArrows` held it whatever the modifiers, `updateHistory` then took no step while it was held, and a burst opened by Cmd + Up in a field waited for a release that never came | reproduced: Cmd + Up on the drawing, then Right pressed and released, moved A from (10, 20) to (11, 19), Up still held, and recorded nothing. The audit named WebKit's bug 110670, which is about a pair of modifiers; read instead, `REF-WEBKIT-CMD-KEYUP` (bug 55291) and `REF-MOZILLA-CMD-KEYUP` (bug 1802465), note 0032 §14. Fixed: an arrow with Ctrl, Cmd or Alt neither held nor opening a burst, the condition `readNudge`, `readArrowStep` and `readZoomKey` used, shared as `isBrowserKey` (`src/model/keys/`); Cmd's `keyup` releases every arrow and ends the burst, its step taken; tests: Cmd + Up with no `keyup`, Up with Ctrl or Alt, Cmd released with arrows held, in `held-arrows.test.ts` and `arrow-bursts.test.ts`; `interaction.md` §2 and §7, Q7, Q59, F39's and F40's guides |
| 2 | Up and Down in X and Y of several shapes went past SP-031's answer 4 (a), which named W and H's example | the Product Owner's choice of 2026-10-24: answer 4 (c) added to (a), no code changed; written in SP-031's answers, US-081, `interaction.md` §2 (The arrows), Q56 and F38's guide |
| 3 | A value typed in the burst's field was dropped as an arrow's: `listenToPanelSteps` took no step for any `change` of that field while the burst was open, so that the arrows and the value typed made one step | reproduced: Up in W of A and B, 120 and 40, then `150` typed within 500 ms: the step still at 120 and 40, the value typed joining the burst's. Fixed, as the Product Owner answered (two steps): a key other than an arrow or a modifier pressed in the burst's field ends the burst, its step taken (`stopTypedBurst`), the value then taking its own; a key in another field keeps the burst, as §7 said. Tests: two steps, Ctrl + Z giving back 121 and 41 first; Shift, Ctrl, Alt, Cmd and Left keeping it; a key in H keeping it; §7, Q7, F40's guide |
| 4 | `listenToRedrawBeforeClicks` was not the canvas's first listener: `listenToInputs` added `listenToPoints`'s click before it | fixed: listened to first in `mountPlayground`, and in the capture phase (`REF-MDN-ADD-EVENT-LISTENER`, its row given `capture`), so that a listener inside the canvas acts after it too; test: at `pointerup` and `click`, a listener on the drawing's layer finds no redraw pending. happy-dom invokes a target's listeners in the order added, the capture phase before them, as the DOM standard does |
| 5 | `STEP_ARROWS` and `FIELD_ARROWS`, Up and Down, written twice; `readArrowStep` turned `nudgeShift`'s y around unnamed | fixed: `fieldArrowStep(key, isFurther)` (`src/model/moving/`, `domain`), the step and the arrows that step a field, used by both; examples and the keys that step nothing |
| 6 | `REF-HAPPY-DOM-QUERY` named happy-dom 21.0.3 where `package.json` pins 21.0.4 | `QuerySelector.js` (`affectsCache`, `findFirst`) and `ParentNodeUtility.js` (`elementIdMap`) read again in the installed 21.0.4, the same; the row's version and date updated |
| 7 | F43's row in E02's file and its benefit hypothesis still name the history's list, gone to E20 with US-083 | shared files, left to the main thread (below) |
| 8 | `PENDING_REDRAWS` outlived a mount: the redraw a mount asked and did not draw swallowed the next mount's requests until its frame | reproduced: a value typed, the page mounted again, then another typed: no frame asked. Fixed: `listenToRedrawBeforeClicks` forgets it as the page is mounted, the mount drawing the page itself, as `listenToHeldArrows` forgets the arrows; test: a frame of its own asked after the mount |
| 9 | EN-150's criterion says 343 redraws, its commit `b16a95aa` 364 | both measured again, the listeners left: 343, 49 a key, for the 48 mounts' case run alone, as SP-031 measured; 364, 52 a key, in the whole file, its 1 and 2 mounts' cases mounting 3 times before. The criterion's number stands |
| 10 | US-080's, US-081's and US-082's tasks and EN-150's were left unticked | ticked, each by its commit |
| 11 | AC1's "in under 5 s alone" and "its hue slider's test in under 200 ms" were bound by no test | measured again, 5 runs alone: the file 3.65 to 3.95 s, the hue slider's test 116 to 124 ms, a median of 120. Not made a timeout: 200 ms is under four times 120, which `[F29.AC1]` refuses for a timing test, and Vitest's 5 s bounds one test, not a file; both stay measures, written here |
| 12 | The band's hole about a box, `min(width, height) > 2 × inner reach` (`readBoxBandPathData`, US-080), was tested on no box near its bound: 2 turned 3 or `×` turned `÷` survived the mutations | test `read-box-band-path-data.test.ts`: a box 4 × 4 hollow, 3 × 10 and 10 × 2.5 filled whole, the inner reach 1.5; fails on 3, on `÷` and on `≥` |

Mutations (A4), `npm run mutate`, the work staged first: the 3 modules of `src/` F43 added or changed with a body, `heldNudgeShift`, `fieldArrowStep` and `isBrowserKey`, against their related tests, 5 caught and none survived. The 48 modules of the playground F43 added or changed with a body, in three groups against the tests of their folders: the keys, the bursts, the history and several shapes' fields (16 modules), 39 caught and 1 survivor, `writeSeveralValues`'s `values.length > 0` turned `> 1`, equivalent, one shape's values being common in every field, so that Mixed is never shown; the redraws, the listeners and the mount (9), 11 caught; the helps, the hover, the picking, the selection drag, the drawing and the wheel (23), 56 caught and 3 survivors: the band's hole bound, twice, finding 12; `readSelectionMove`'s `?? 0` turned `?? 1`, equivalent, a shape selected always having its pick area. One file given was gone, `create-outline-element.ts`, which F43 folded into `create-outline-elements.ts`; the tool skipped it. By hand, each criterion once: the mount's `abort` dropped (`[F43.AC1]`, 4 fail in `remove-listeners.test.ts`, the case of 48 mounts at 343 redraws, 364 in the whole file); the white band 2 pixels wide instead of 4 (`[F43.AC2]`, the hover's, the selection's and the rectangle's tests fail, `outlines A under the pointer by a white band 4 pixels wide under the accent's 2` among them); `heldNudgeShift` keeping the last arrow alone (`[F43.AC3]`, the examples, the property and `moves A by (+1, −1) at each key from Right's first` fail); a text's box never squared (`[F43.AC4]`, the three tests of a text's square fail, with F36's circle). And the new tests: each seen failing with its fix undone, `isBrowserKey` dropped from the arrows held (3) and from the bursts (2), Cmd's release left out of both (1 each), the typed key not ending the burst (1), the modifiers or the arrows counted as typed (1 or 2), a key in another field ending it (1), the canvas's capture dropped (2), the pending redraw kept across a mount (1), `fieldArrowStep`'s sign turned (6 and more in `write-several-values.test.ts`), Cmd not read by `isBrowserKey` (3), and the band's bound at 3, `÷` and `≥`.

Properties (ADR-0022): `fieldArrowStep` and `isBrowserKey` are `domain`, their examples covering every key and modifier they read; F43's `heldNudgeShift` keeps its own.

Breadth (step 7): `check:all` passes with the push; `AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, `docs/conventions/` and `docs/tooling/` still describe what F43 did; `exceptions.md` gains no row.

Next feature (G): F43 closes E02's corrections; the next is the main thread's to choose. Left for later review, not fixed here: a value pasted in the burst's field by the mouse, which sends no key, still joins the burst's step; Chromium's issue 1393524, named by WebKit's bug, not read.

Evolvability (H):

- Size: `src/model/keys/` a new domain of 1 module and its type; `src/model/moving/` 7 modules; `playground/arrow-bursts/` 4.
- Tools: `npm run mutate` skips a file given that is gone, and goes on with the others; nothing to fix.
- Fitness functions: AC1's two times are measures, not tests, `[F29.AC1]`'s four times forbidding a 200 ms bound on a test of 120 ms (finding 11).
- Patterns: a key held from its `keydown` needs every way its release may be lost: the window's `blur` (AUD-044) and, on macOS, Cmd (finding 1); a record kept by document is forgotten at each mount (finding 8).
- Agent configuration: nothing.
- Process: a source named by an audit is read before it is cited (finding 1).

## For the main thread

- For the done commit's footer: the rule docs changed in this task, `interaction.md` §2 (the arrows held, those in X and Y) and §7 (the burst ended by a value typed and by Cmd), the Q rows Q7, Q56 and Q59 of `docs/domain/README.md`, and the guide pages of F38, F39 and F40; F43 has no guide page of its own and VAL-044 is dropped.
- Finding 7, shared files: F43's row in `docs/backlog/epics/E02-edit-shapes.md` and F43's benefit hypothesis in its feature file still name "the history's list", which went to E20 with US-083 (SP-031's answer 1): to remove.
