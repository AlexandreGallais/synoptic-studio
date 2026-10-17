---
id: AUD-017
epic: E15
feature: F16
title: Audit F16
status: done
points: 2
---

# E15 · F16 · AUD-017 — Audit F16

The feature in depth, the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given the functions of F16, when audited, then every formula of `DERIV-color-spaces` is re-derived, every expected value recomputed by hand, the round trip re-run over every color, and each function survives a mutation spot-check.
- Given the sources, when audited, then every `@see` of F16 is re-verified and dated.
- Given the project, when audited, then no duplicate, no copied code, no unnamed calculation (ADR-0049) and no inconsistency between code, domain, ADRs and docs remains unrecorded.

## Tasks

One task = one commit, referenced as `AUD-017.Tn`.

- [x] T1 — The audit by independent reviewers and its findings (2 h)

## Findings

| Check | Result | Evidence | Action |
| --- | --- | --- | --- |
| A1 — formulas of `DERIV-color-spaces` | hold | auditor and math-expert re-derived steps 3 to 11 from CSS Color 4 §7.1, §7.2 and Wikipedia (HSV alternative, From RGB) before reading the code; step 7 by hand (`9 − k ≥ 3`, not `> 3`, at h = 1); step 11 bound 1/(2D) = 8.3 · 10⁻¹⁰ against 2.8 · 10⁻¹⁴ | step 7 inequality corrected |
| A1 — round trip over every color | hold | math-expert, 16 777 216 colors: unrounded 0 / 0, one decimal 0 / 0, picker path (`hexToPickedColor` then `pickedColorToHex`) 0 / 0 in HSL / HSB; auditor the same picker path 0 / 0; step 8 bounds 3.69 and 3.78 · 10⁻¹³ | picker-path result added to the cross-check |
| A2 — expected values | all correct | every value of `src/math/color`, `src/math/exact-color`, `src/io/color`, `src/io/picked-color`, `src/model/color-picker` and the US-035 test card recomputed by two reviewers; `10 · 30.1` is 301 exactly, not 301.00000000000006 as `to-tenths.test.ts` said; ⌊50.696⌋, not ⌊50.69⌋ | two comments corrected |
| A3 — model switch keeps the hue of any color | defect | auditor: HSB 0.7°, 30 %, 32.2 % gives `#523a39`; HSL chosen showed 0.7°, 18 %, 27.3 %, which gives `#523939`; 20.3 % of a grid of picked values changed; reproduced | fixed: the hue kept only while the values give the color back (`show-model-chosen.ts`), test first; provisional, VAL-016 decision 17 |
| A3 — ties of the values shown | precision | math-expert: `#002330` hue 196.25° computed 196.24999999999997, shown 196.2; `#030330` saturation 93.75 % shown 93.7; 27 337, 20 244 and 28 548 exact ties shown down; reproduced; either neighbour gives the color back | rule written as nearest tenth as computed, provisional, VAL-016 decision 19; exact option logged |
| A4 — mutations | 61 of 67 caught at once | 67 mutations in a copy of the repository, one per function at least: e.g. `hue` corner sign → `[F16.AC3] gives back the components HSL was read from`; `FALL_END` 2400 → 2300 → `gives 330900000 / 6 · 10⁸ for n = 3`; `parseHexColor` without trim → `reads '  #ff8000' …` | — |
| A4 — survivors | killed | `toShownHsb` brightness and `toShownHsl` lightness unrounded, `hexToPickedColor` grey test by `shown.hue === 0`, the opacity slider's range and thumb maximum, the plane mark not turned, a typed lower bound, an opacity overwritten after a color without alpha, Escape guard; `toTenths` truncating equivalent on its stated input (`10 · (i / 10) = i` for every tenth to 3600) | 12 tests added, each mutation re-run and caught |
| A5 — properties | math complete | every `math` function of F16 has a `test.prop`; `format` functions without one listed | logged for NAM-001 |
| B1 — sources | verified 2026-10-15 | `REF-CSS-COLOR-4` §5.1, §5.2, §7.1–7.3, §16.1.1, §16.2.1, §16.2.2; `REF-CSS-VALUES-4` §5.2; `REF-WIKI-HSL-HSV`; `REF-MDN-MATH-ROUND`; `REF-ECMA-262-NUMBER-REMAINDER`; `REF-MDN-REMAINDER`; `REF-WAI-APG-SLIDER`; `REF-FIGMA-COLOR-MODELS`, `-PICKER`; `REF-INKSCAPE-1-2-COLOR`, `-1-3-COLOR` | byte rounding cited to §5.1, not §16.2.2 (decimal serialization); MDN quote added to its row |
| B2, B3 — links, unverified | green | pre-push `check:all` (docs build, references test); no `@see` to an `[unverified]` row | — |
| C — provenance | clean | CSS sample code not copied (`modulo` and `clamp`, no `1e−5` threshold); each TSDoc formula matches its body; no dependency added | — |
| D1 — duplicates | found | `percentToOpacity` and `fractionFromPercent`; `percentToAlpha` and `ratioToByte`; `toOneDecimal` and `toTenths`; 255 declared five times | logged for NAM-001 |
| D4 — unnamed calculations | found | byte to fraction in `hexToRgb` and `rgbOfBytes`; `min(l, 1000 − l)` in `hslComponentInTenths` | logged for NAM-001 |
| E1 — consistency | aligned | AGENTS.md said the picker was next; the background's use of the color fields stated as settled in `interaction.md` and `shapes.md`; the plane's End key missing; seven provisional rules missing from VAL-016; a phrase of the refinement quoted in note 0019 | fixed; VAL-016 decisions 14 to 20 |
| E3, F4 — agent configuration | aligned | `.claude/rules/library.md` pitfalls match; "behaviour" spelled "behavior" as elsewhere | fixed |
| F — quality gates | green | `npx tsc --noEmit`, `npm run fix`, `npm run golden:check`, pre-push `check:all` | — |
| G — next feature | listed | E15 continues with its validation VAL-016, then E02; for the shapes' fills and strokes (E04) the same fields and picker, with SVG `fill-opacity` and `stroke-opacity`, their SVG 2 painting section a source to read at its spike | — |
| H — evolvability | noted | size: `src/io/color` 13 modules, near the 15 signal; tools: scripted mutations in a copy worked again; fitness: golden check by hand; patterns: exact integers beside floats, a fit; agent configuration: unchanged; process: reviewers in parallel found the one defect | logged |
