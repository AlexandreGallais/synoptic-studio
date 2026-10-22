---
id: AUD-037
epic: E02
feature: F35
title: Audit F35
status: ready
points: 1
---

# E02 · F35 · AUD-037 — Audit F35

Audit F35 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F35's criteria, when the audit ends, then each is traced to a test titled `[F35.ACn] …`, broken once on purpose and seen failing.

## Tasks

- [x] T1 — Criteria traced, findings fixed (1 h)

One task = one commit, referenced as `AUD-037.T1`.

## Findings

The `auditor` alone reviewed F35 read-only (ADR-0086); each of its six findings was reproduced before it was fixed. F35's one criterion is traced to the `[F35.AC1]` tests of `centred-position.test.ts`, `drawing-with-shape.test.ts`, `place-shape.test.ts`, `toolbar.test.ts` and `f05-steps.test.ts`, each broken by a mutation below.

| Id | Finding | Result |
| --- | --- | --- |
| 1 | Reset and a guided step without a text kept the last text's options (`write-default-shape.ts`, `write-step-values.ts`), against `interaction.md` §1, Q45, VAL-036 and the guide page: a text of size 20, weight 700, then Reset, gave a text 20 Bold | fixed: every step and Reset write the Text tool's fields, the page's (`OPENING_TEXT`, shared with the F05 steps) when the step has no text; `[F35.AC1] forgets the last text's options on Reset: the next text is Inter 32 Regular`, Reset and the next step, seen failing before the fix |
| 2 | A text whose font has no metrics is placed with its top-left on the point, a rule only in the code (`write-placed-text.ts`) | written in `interaction.md` §1 as the agent's choice, in Q45 and in VAL-036's decisions to confirm; `[F35.AC1] places a text whose font has no metrics with its top-left on the point` |
| 3 | A kept text whose metrics are no longer read is dropped from the drawing and the export (`create-drawing-elements.ts`) | it can happen: Inter not drawn (US-056), a family read ahead and measuring the text is removed from the session when its files cannot be read again (`write-default-choice.ts`, US-061); fixed: a kept text is drawn with the metrics it was last drawn with (`KEPT_TEXT_METRICS`), written in `interaction.md` §1, Q45 and VAL-036; `[F35.AC1] keeps a text whose font's metrics are no longer read, drawn and exported as it was`, seen failing before the fix; a kept text never measured draws nothing (`[F35.AC1] draws no kept text that no metrics ever measured, …`) |
| 4 | TSDoc still describing US-032's per-tool memory (`choose-shape-kind.ts`, `guided-steps-f05-options.ts`) | fixed, and the same words in `write-text-step-values.ts` |
| 5 | `read-drawing-point.ts` repeated the pointer-on-element calculation of `read-pointer-offset.ts` (ADR-0049) | fixed: `readPointerOffset` moved to `playground/page/`, taking any mouse event, and called by `readDrawingPoint` and the color picker's drag |
| 6 | F35's row of `docs/guide/README.md` sat before F33 and F34 | fixed: after F34 |
| 7 | Mutation: `write-placed-text.ts:43`, the height of a text without metrics 0 → 1, survived (the click at y 140.4 rounded alike) | fixed: the test clicks at (250.5, 140.6), (251, 141) |
| 8 | Mutation: `create-drawing-elements.ts:35`, `[]` → `undefined`, survived | fixed: the test checks no stray word is drawn in the text's place |
| 9 | Mutation: `write-step-values.ts:73`, the angle 0 → 1, survived: the Text tool's fields, now written at every step, write the angle after it | fixed: the line removed, its comment moved |

Mutations (A4), `npm run mutate`, after the fixes: `centred-position.ts`, `drawing-with-shape.ts`, `is-drawn-text.ts`, `shape-corners.ts` (`--all` on the first and the last) against their own tests, 7 caught; `add-placed-shape.ts`, `write-placed-text.ts`, `write-placed-box.ts`, `listen-to-placing.ts`, `read-drawing-point.ts`, `create-drawing-elements.ts`, `read-stored-drawing.ts`, `save-kind-settings.ts`, `write-kind-settings.ts`, `read-armed-tool.ts`, `write-step-values.ts` against `place-shape.test.ts` and `toolbar.test.ts`, every one caught once findings 7 to 9 were fixed. By hand: `centredPosition` rounding x down (4 tests fail) or halving the width for the height (2 fail); `drawingWithShape` inserting the shape second (the new property fails). `write-placed-box.ts` has no mutation the tool writes.

Properties (ADR-0022): `drawingWithShape` gets one (the shape last, the others as they were, the drawing unchanged); `centredPosition` and `shapeCorners` had theirs; `isDrawnText`'s example covers each kind of shape.

Next feature (G): F36 needs how Figma and Inkscape outline the shape under the pointer and select by a click (a hover layer, a click on the empty drawing), and Delete and Backspace outside a field; `REF-FIGMA-SHAPE-TOOLS` covers the drag, the rest is for F36's spike.

Evolvability (H):

- Size: `playground/guided-steps/epic-15/` is at its 15 modules, so the opening text went to `guided-step/`; nothing yet.
- Tools: no "considered for later" row is triggered by F35.
- Fitness functions: what Reset forgets is written by hand in each place the page keeps a state (fields, data attributes, module maps), which findings 1 and 3 show; a test that Reset gives the page as it opens, field by field, is worth a look at the next review.
- Patterns: the pointer-on-element calculation was the one duplicate (finding 5).
- Agent configuration: `AGENTS.md` and `CLAUDE.md` hold 158 lines together; `npm run mutate`'s message for a skipped file hides why (see the incident below).
- Process: nothing.

## For the main thread

### Incidents

#### 2026-10-22 — epic-check, to read its usage

- Used: `target/release/epic-check --help`, by the auditor of AUD-037, to read its options.
- Failed: it ignores `--help` and runs the whole epic check, `check:all` included, in the worktree it was called from.
- Instead: the check was left to end; it left no file in the worktree, as `git status --ignored` showed at the fixes' start.

#### 2026-10-22 — mutate, for the mutation spot-checks of F35

- Used: `npm run mutate -- <files>`, the audit's new modules not yet committed.
- Failed: the copy takes tracked files only, so every test importing a new module failed in it, and each file was skipped as "its related tests fail, or none was found", the cause unsaid.
- Instead: the new modules staged with `git add` before the run, which the copy then took.
