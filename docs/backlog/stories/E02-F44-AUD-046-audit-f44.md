---
id: AUD-046
epic: E02
feature: F44
title: Audit F44
status: done
points: 1
---

# E02 · F44 · AUD-046 — Audit F44

Audit F44 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F44's criteria, when the audit ends, then each is traced to a test titled `[F44.ACn] …`, broken once on purpose and seen failing.

## Tasks

- [x] T1 — A text typed kept apart from the value applied (findings 1 and 2): the number a relative value starts from read at the focus and at each key while no text typed waits in the field, a redraw reading the value applied; the test helper typing as a browser does, a key per character and for Enter (1 h)
- [x] T2 — The rule for every number field (findings 5 and 9 to 20): a value beyond a field's range taken at its bound or wrapped, a fraction rounded once at the end, only a text that is no number refused; `interaction.md` §2 and §7, Q56, US-085 AC4 and US-087 AC2 (2 h)
- [x] T3 — The number fields' mechanism made one (findings 3 and 4), the rest of the findings, the criteria traced and broken by hand, the mutations (2 h)

One task = one commit, referenced as `AUD-046.Tn`; split in three on 2026-10-24 as the work outgrew one commit.

## Findings

The `auditor` alone reviewed F44 read-only (ADR-0086); each finding below reproduced before it was fixed, the Product Owner's rule for every number field of 2026-10-24 settling findings 5 and 9 to 20, the main thread choosing the hue's going round and the relative value with no number to start from. F44's four criteria are traced to the `[F44.AC1]` tests of `canvas-size.test.ts`; the `[F44.AC2]` tests of `edited-fields.test.ts` and `one-shape-bursts.test.ts`; the `[F44.AC3]` tests of `helps-on-any-color.test.ts`; and the `[F44.AC4]` tests of `own-number-fields.test.ts`, `radius-field.test.ts`, `write-several-values.test.ts` and the library's `panel-value-after-edit.test.ts`, `whole-value-with-typed.test.ts`, `value-with-typed.test.ts`, `parse-panel-value.test.ts` and `radii-with-typed.test.ts`, with the tests of each field the rule changed, each broken below.

| Id | Finding | Result |
| --- | --- | --- |
| 1 | `writeShownNumber` stored, at every `keydown`, whatever the field showed, the text typed included, as the number a relative value starts from; `typeInField` sent one key before the text and none per character nor for Enter, so that no test saw it | reproduced with the helper typing as a browser does, a key per character and one for Enter: `-5` in the Angle at 30 gave 350, the number shown at Enter's key being −5; `5` typed in the radius at 10, then Up, gave 6. Fixed: the text shown read at the focus and at each key while no text typed waits in the field, the field marked typed at its `input` until a value is applied or the page writes one (`listenToTypedNumbers`, `SHOWN_TEXTS`, `TYPED_FIELDS`, `playground/applied-values/`); tests: `-5` in the Angle at 30 gives 25, `5` typed then Up gives 11, `+5` typed then Up 11; `interaction.md` §2, Q56 |
| 2 | Every redraw read the fields' text, so that a text typed and not applied was drawn by a wheel's zoom or pan, a font loaded or the canvas resized, and `+10` typed refused the shape | reproduced: 150 typed in Width and a wheel's pan drew the rectangle 150 wide, `+10` refused it; 30 typed in the Angle drew it turned. Fixed: the page reads the value applied while a text typed waits (`readShownText`, read by `readNumber`, the free shape's sizes and a text's X and Y); tests: 150 and `+10` typed in Width then a wheel, the rectangle 120 wide and no message; 30 typed in the Angle, the shape at 0° |
| 3 | `listenToNumberField` and `listenToOwnNumberFields` did the same three things, `applyArrowStep` and `applyArrowValue` the same, and `readAppliedNumber` wrote `panelValueAfterEdit`'s formula again unnamed | fixed: one mechanism heard on the document for every field acting as a number field (`listenToNumberFields`), its apply alone differing, the shapes' kept by field (`NUMBER_FIELD_APPLIES`, `listenToNumberField`) or the field's own (`applyOwnText`, `writeOwnValue`); one arrow handler (`applyArrowValue`); the formula named `valueWithTyped`, called by `wholeValueWithTyped`, so by `panelValueAfterEdit`, and by `readOwnValue`. Every test of the fields passed unchanged across the change |
| 4 | The bounds `data-min` and `data-max` of `index.html` copied TypeScript constants (`TEXT_RANGES`, Q19's 3 to 12, `HISTORY_BOUND_FIELDS.most`); the zoom had no `data-min`, the picker's fields no bound | fixed: every field of its own has its range in a table built from the constants of what it sets (`OWN_FIELD_RANGES`: `TEXT_RANGES`, `REGULAR_POLYGON_CORNER_RANGE`, `HISTORY_BOUND_FIELDS`, `PERCENT_SLIDER_RANGE`, `MAX_SIZE`, `DEGREES_PER_TURN`), the Zoom from 1 %, the picker's fields bounded, the page writing none; test: the table names every field of its own of the page, Corners from 3 to 12 |
| 5 | The picker's H, S, B or L and the Zoom no longer took a fraction | settled by the rule: a fraction is accepted everywhere, rounded once the value is computed in a field of whole numbers, kept in the Zoom's (141.5 zooms to 141.5 %) |
| 6 | The tests' stand-in `ResizeObserver` does not call back at `observe`, where `REF-RESIZE-OBSERVER-1` §1 says an observation fires as it starts for a box not 0 × 0 | the stand-in kept, so that the tests' counts of redraws stay a mount's own (EN-150's 343); the browser's one draw more at a mount's first frame stated as accepted in `listenToCanvasSize` and the stand-in |
| 7 | US-086's tasks were left unticked and its T3 said `readBoxBandPathData` stays for a text's frame, gone with the bands (`0a1252c3`); US-087's AC4 assumed fields stepping by other than 1 | US-086's tasks ticked, its T3 corrected; US-087's AC4 amended: every field steps by 1 |
| 8 | `readNumber`'s `valueAsNumber` branch served no field; `src/geometry/contour-band/` and `DERIV-band-within-box` have no caller | the branch removed, every input of the page a text field (T1); the bands kept, as SP-032 decided, for E04: the card is the main thread's (below) |
| 9 | `parsePanelValue` refused a fraction and an integer past the safe ones | the rule: a decimal with a sign or none read as typed, its fraction kept for the field to round (`50.5`, `+1.5`, `-.25`, `5.`), a comma a decimal separator as a point (`50,5`, `+1,5` at 10 giving 12; the Product Owner's, 2026-10-24, sent during T3 and committed with it); refused alone what is no number: an empty field, `.`, `1,2,3`, an exponent, a unit, `0x10`, `Infinity`, a number too large for a double, an expression such as `-25+12`, left to E18's number fields that compute |
| 10 | `panelValueAfterEdit` refused a width or height below 0 | the rule: taken at 0, rounded first when a fraction is typed (`wholeValueWithTyped`, `nearestInteger`, Q45): 40 − 41 gives 0, 99 999.6 gives 100 000, −0.4 gives 0; an integer typed keeps a shape's own fraction, a converted triangle 50√3 high giving 50√3 + 10 for `+10` (Q27), so that the result is rounded only when a fraction is typed |
| 11 | `shapeAfterPanelEdit` and `textAfterPanelEdit` passed on that refusal | they refuse nothing now, their type without `undefined`; `readEditedKept` and `applySeveralValue` follow: `-50` in W of A and B, 100 and 40 wide, makes them 50 and 0, where it changed none |
| 12 | `readOwnValue` refused a value past `data-min` or `data-max` | the rule: taken at its bound, a turn going round: 13 in Corners gives 12, 2 gives 3, Up at 1 000 MB stays 1 000, 1 001 MB gives 1 000, `-15` in the radius at 10 gives 0 |
| 13 | `listenToZoomMenu` refused 0 and below | the rule: taken at 1 %, by the field's range; 0 zooms to 1 %, `-50` at 200 % to 150 %, a fraction kept |
| 14 | `readBoundPart` refused a fraction or a value past its field's most | the field takes it first, at its bound and rounded, 2.5 MB giving 3 and 101 steps 100; `readBoundPart` still refuses a value read from the browser's storage, as before |
| 15 | `commitOpacityField` refused an opacity past 0 to 100 or not whole | the field takes it first within 0 to 100, rounded: 101 in the underline's opacity gives 100 and marks its color chosen, as any opacity applied |
| 16 | `readTypedCoordinate` refused a fractional X or Y | rounded, a tie up (`nearestInteger`); the value reaches it rounded already, the field's value being rounded once computed |
| 17 | `readShape`'s messages, `TEXT_RANGES` and `SHOWN_NUMBER` reached only by a field showing no number once fields take every number | kept: a shape is still refused when its points span no area (Q33), or when a field shows no number after a script of the page emptied it, as the tests that check a control on a refused shape do (library rule, US-014); in a browser no value typed refuses a shape (open point below) |
| 18 | `listenToPickerValues` held the hue within 0 to 360 | the main thread's choice: a hue is an angle, going round: 400 gives 40, −10 at 0 gives 350, 370 gives 10 (`degreesInTurn`); the field's range does it first |
| 19 | `listenToTurn` already went round | unchanged; the Angle's range goes round too, so that `-20` at 10 gives 350 before its listener |
| 20 | `applyEditedValue` refused a relative value in a refused shape's field | the main thread's choice: from 0, a size within 0 and 100 000: `+30` in a refused shape's Width gives 30, `-30` gives 0; a text that is no number puts back the text shown (`resetShownText`). In a Mixed radius, a relative value or an arrow's step goes to each vertex's own radius (`radiiWithTyped`): radii 10, 0, 0, 0, 0 and 6 and `+5` give 15, 5, 5, 5, 5 and 11, Up 11, 1, 1, 1, 1 and 7, `+0` none, `abc` Mixed shown again; in a point's Radius, the vertices selected alone |
| 21 | Found in fixing 10: rounding a fraction typed moved a turned text typed back with the 5 decimals X shows, which AUD-040 keeps exact | a text typed again as the field showed it applies nothing, as a browser sends no `change` for a field left unchanged (`stopUnchangedText`, `REF-MDN-CHANGE-EVENT`); test: 46.71377 typed back, the exact position kept |

Mutations (A4), `npm run mutate`, the work staged first: the 8 modules of `src/` F44 added or changed, against the tests of `src/model/panel-values/`, `src/io/panel-value/` and `src/model/selection/`, 12 caught and 1 survivor, `parsePanelValue`'s `return undefined` replaced by `undefined`, the same code. The 58 modules of the playground F44 added or changed, in two groups against the tests of their folders: the number fields, the applied values, one shape's and several's fields, the radius, the zoom, the picker and the text's position (43 modules), 60 caught and 1 survivor, `readOwnValue`'s 0 to start a fixed value from turned 1, equivalent, a fixed value reading no number; the helps, the hover, the drawing, the view and the mount (15), 26 caught and 4 survivors: `readInsetBoxPathData`'s half a box, `/ 2` turned `* 2` or `/ 3`, a box narrower than its inset tested by none, now `read-inset-box-path-data.test.ts`, failing on both; and two lines F44 did not write, `showShape`'s JSON indent 2 turned 3, a text of the pipeline, and `showRefusedShape`'s guard on the radius field, which is always an input, equivalent. The 5 files of the guided steps hold texts alone. By hand, each criterion once: the canvas left unobserved (`[F44.AC1]`, 7 of `canvas-size.test.ts` fail); one shape's fields given no apply (`[F44.AC2]`, 34 of `edited-fields/` fail); the helps' dashes dropped (`[F44.AC3]`, 13 of `helps-on-any-color.test.ts` fail); Shift read by no arrow of a field (`[F44.AC4]`, Shift + Up in the radius and in a text's size and the memory fail). And the new tests: each seen failing with its fix undone, the shown text read at every key (9, `-5` in the Angle, `5` then Up, every refusal), the redraw reading the text typed (the wheel's 2 and the Angle's), the range's clamp dropped (12), the turn clamped (6), the fraction not rounded (14), a size below 0 not taken at 0 (7), the decimal's point refused (3), the comma read as no number (5), the refused shape's value not from 0 (1), the text typed again applied (1), the Mixed radius set whole (2) or read fixed (1), `+0` written `0` (1), the radii not taken at 0 (2).

Properties (ADR-0022): `valueWithTyped`, `wholeValueWithTyped` and `radiiWithTyped` are `domain`, their examples covering a sign, none, a fraction, a tie, a bound and a fraction of the own value; `parsePanelValue` keeps its property.

Breadth (step 7): `check:all` passes with the push; `AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, `docs/conventions/` and `docs/tooling/` still describe what F44 did; `exceptions.md` gains no row.

Next feature (G): F44 closes E02's corrections from the second try; the next is the main thread's to choose. Left for later review, not fixed here: a redraw meanwhile rewrites the Radius and a point's fields, written by the page with the radii applied, dropping a text typed in them (finding 2's text is kept in the other fields); a shape refused is reached in the tests by emptying a field by script, a path a browser never takes, the only one left besides points spanning no area (finding 17).

Evolvability (H):

- Size: `playground/applied-values/` a new folder of 6 modules; `playground/number-fields/` 13; `src/model/panel-values/` 14.
- Tools: `npm run mutate` reports an equivalent `return undefined → undefined`; nothing to fix.
- Fitness functions: the test over every number field now also finds each field of its own in `OWN_FIELD_RANGES`, so that a field added without its range fails.
- Patterns: a test helper sends the events a browser sends, a key per character and for Enter (finding 1), or a fault of the keys stays unseen; a page that reads a field at a redraw reads the value applied, never the text typed (finding 2).
- Agent configuration: nothing.
- Process: a rule of the Product Owner for every field is applied to each refusal the audit listed, each with its test, and the criteria amended with it (US-085 AC4, US-087 AC2).

## For the main thread

- For the done commit's footer: the rule docs changed, `interaction.md` §2 (A value typed, A value beyond a range taken at its bound, One shape's fields, Every number field), §5 (the picker's hue going round) and §7 (the percentage typed, A shape refused, the bound's fields), `shapes.md` §3 (a text's box fields and its X and Y), the Q rows Q7, Q56 and Q60 of `docs/domain/README.md`, and the guide pages of F38, F40 and F42; F44 has no guide page of its own and VAL-045 is dropped.
- Finding 8: a card in E04 for `src/geometry/contour-band/` and `DERIV-band-within-box`, kept without a caller by SP-032's decision for a shape's stroke.
- The comma as a decimal separator, sent by the main thread during T3, is committed with T3 rather than with T2, the rule's task, T2 being committed already.
