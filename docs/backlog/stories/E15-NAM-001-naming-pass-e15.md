---
id: NAM-001
epic: E15
title: Naming pass over the code of E15
status: ready
points: 3
---

# E15 · NAM-001 — Naming pass over the code of E15

Before the review, go over the whole code base twice so that every calculation and every process that a field names is a function of that name, with its source (ADR-0049).

## Acceptance criteria

- Given the whole code base, when the first pass ends, then every operation or run of steps that a field names is a call to a function of that name with its `@see`, and each one left inline is listed with its reason.
- Given the result of the first pass, when the second pass ends, then what the first missed and the duplicates found are named or merged, unless merging changes a floating-point result.
- Given a golden file written before the first pass (`npm run golden:write`), when each pass ends, then `npm run golden:check` passes and no expected test value was edited.

## Tasks

One task = one commit, referenced as `NAM-001.Tn`.

- [x] T1 — Golden file; list the unnamed calculations and their sources (1 h)
- [ ] T2 — First pass (3 h)
- [ ] T3 — Second pass and duplicates (2 h)
- [ ] T4 — Audit of both passes (1 h)

## Golden files

`npm run golden:check` passes on the base (US-055's branch, 2026-10-16), its file written at F13 and never written again. It draws shapes only; for E15's results the story keeps a second golden file outside the repository, written once on the base: 150 texts drawn with Inter (every option, wrapped, cut, turned), every color conversion on a grid of 1 500 colors and the picker's plane, and 48 system font files read (faces, metrics, kerning, a collection's parts by their digest).

## First pass: the calculations to name

Each row is a calculation written bare, or twice, with the name and the source it gets. Each one is on integers, or keeps its operations in the same order, so that no result changes.

| Folder                                                       | Written bare                                                                                                                       | Named                                             | Source                                  |
| ------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------- | --------------------------------------- |
| `io/character-map`, `font-name`, `font-file`, `kerning`      | `start + size · i`, the place of a record in an array, about 30 times                                                              | `elementAddress`                                  | `REF-WIKI-ARRAY` (new)                  |
| `io/kerning`, `model/kerning`                                | `class2Count · class1 + class2` (a Class2 record), `first · 65 536 + second` (a pair's key)                                        | `rowMajorOffset`                                  | `REF-WIKI-ROW-MAJOR` (new)              |
| `io/character-map`, `io/kerning`                             | the index in a range: `startGlyphID + c − startCharCode`, `startCoverageIndex + g − startGlyphID`, `g − startGlyphID`, `c − start` | `indexInRange`                                    | `REF-OT-LAYOUT-COMMON`                  |
| `io/character-map`, `font-file`, `font-name`, `math`         | `⌊n / s⌋`: the whole records that fit, the code units of a name, the high bits of a byte                                           | `integerQuotient`                                 | `REF-WIKI-FLOOR-CEILING` (new)          |
| `math/arithmetic`                                            | `value.toString(2).length` in `populationCount`                                                                                    | `bitLength`                                       | `REF-WIKI-BIT-LENGTH` (new)             |
| `io/font-tables`                                             | the directory's size, `12 + 16 · numTables`, twice; the padding `end − start − length` beside `lengthToFourByteBoundary`           | `tableDirectorySize`, `paddingToFourByteBoundary` | `REF-OT-OTFF`                           |
| `io/font-tables`, `font-file`, `font-face`, `font-embedding` | a span that fits, `offset + length ≤ size`, written beside `isSpanWithin`                                                          | `isSpanWithin`                                    | `REF-WIKI-INTERVAL`                     |
| `io/font-tables`                                             | `parseFaceTables` reads the directory as `parseTableDirectory` does                                                                | `parseTableRecords`, shared                       | `REF-OT-OTFF`                           |
| `io/color`                                                   | byte to fraction, `b / 255`, in `hexToRgb`                                                                                         | `byteToFraction`, inverse of `fractionToByte`     | `REF-CSS-COLOR-4`                       |
| `io/color`, `io/picked-color`                                | `round(k · N / D)` for k = 255 (`ratioToByte`) and k = 100 (`ratioToPercent`, `alphaToPercent`)                                    | `ratioOnScale`                                    | `DERIV-color-spaces`                    |
| `io/picked-color`                                            | the hue's fraction divided inline                                                                                                  | `ratioValue`                                      | `DERIV-color-spaces`                    |
| `math/exact-color`                                           | `min(L, 1 − L)` counted in integers: `Math.min(sum, 510 − sum)`, `Math.min(l, d − l)`                                              | `exactLargestHalfChroma`                          | `REF-WIKI-HSL-HSV`                      |
| `io`, `math`                                                 | 255 declared in five files, 100 in six                                                                                             | `BYTE_MAXIMUM`, `PERCENT`, each declared once     | `REF-CSS-COLOR-4`                       |
| `io/text-style`                                              | `percentToOpacity` beside `fractionFromPercent`                                                                                    | merged into `fractionFromPercent`                 | —                                       |
| `model/text-box`                                             | `⌊(H − f) / λ + ε⌋ + 1`, the lines whose heights fit a box                                                                         | `progressionTermsUpTo`                            | `REF-WIKI-ARITHMETIC-PROGRESSION` (new) |
| `model/text-layout`, `geometry/layout-box`                   | three tables of the room before (`ROOM_BEFORE`, `ACROSS`, `FACTORS`)                                                               | `alignedOffset`, on `alignmentFactor`             | `DERIV-regular-polygon-fit`             |
| `model/line-characters`, `model/text-block`                  | the code point −1 before a line's first character, kerned as a pair whose row is zeros                                             | only pairs of neighbours kerned, no sentinel      | `REF-OT-KERN`                           |
| `model/font-embedding`, `model/font-import`                  | weight and style compared in `isSameFace` and in `facesWithFace`                                                                   | `isSameStyle`                                     | `REF-CSS-FONTS-4`                       |
| `model/font-import`, `model/font-embedding`, `io/text-style` | the first of each family kept three ways (`distinctFamilyNames`, `formatFontFamily`, `usedFaces`)                                  | `classRepresentatives`                            | `REF-WIKI-EQUIVALENCE-CLASS`            |
| `model` (E01)                                                | `Number.isSafeInteger(v) && v ≥ a && v ≤ b` beside `isInIntegerRange`, four times                                                  | `isInIntegerRange`                                | `REF-WIKI-INTERVAL`                     |
| `playground/drawing`, `playground/default-font`              | a box inset on each side; a width within a tolerance                                                                               | `insetBox`, `isWithinTolerance`                   | `REF-WIKI-PARALLEL-CURVE`, the stories  |

## Left inline, with their reason

| Where                                                          | Calculation                                   | Reason                                                                                |
| -------------------------------------------------------------- | --------------------------------------------- | ------------------------------------------------------------------------------------- |
| `blockHeight`                                                  | `n · λ − (λ − f)`, the term `f + (n − 1) · λ` | written in another order, another float; the function names it already                |
| `lineBaseline`                                                 | `top + i · λ + offset`                        | the same: a term in its own order, named by its function                              |
| `measuredTextDrawing`, `positionedLines`                       | `frame.y + offset`, `text.x + offset`         | a coordinate moved by an offset, named by its variable (`top`, `left`)                |
| `sliderValueAt`, `sliderPositionOf`                            | `(max · p) / L`, `(L · v) / max`              | each function is named after its concept; the order keeps the values shown (AUD-017)  |
| `hsbComponent`, `hslComponent`, `exactHsbRamp`, `exactHslRamp` | the ramp `k`                                  | each line is a step of the cited formula (`REF-CSS-COLOR-4` §7.1, `REF-WIKI-HSL-HSV`) |
