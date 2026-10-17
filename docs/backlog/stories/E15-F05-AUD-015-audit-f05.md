---
id: AUD-015
epic: E15
feature: F05
title: Audit F05
status: done
points: 2
---

# E15 · F05 · AUD-015 — Audit F05

The feature in depth, the project in breadth (`docs/conventions/audit.md`).

## Acceptance criteria

- Given F05, when audited, then every check of `docs/conventions/audit.md` leaves evidence, unnamed calculations included (ADR-0049).

## Tasks

One task = one commit, referenced as `AUD-015.Tn`.

- [x] T1 — The audit and its findings (2 h)

## Findings

Four independent reviews of the whole feature (`develop...feature/f05-text`), 2026-10-15: the `auditor` (A to E), the `math-expert` (derivations, expected values, precision), the `verifier` (tests) and the `doc-editor` (pages changed). Every formula re-derived and every expected value recomputed by at least two of them: none is wrong. Each finding below was reproduced before being recorded.

### Fixed in this audit

| Check | Finding | Evidence | Action |
| --- | --- | --- | --- |
| A3, crafted font | A `GPOS` class pair subtable of 0-byte records (both value formats 0) passed the span check and built `class1Count × class2Count` values, up to 4.3 × 10⁹, from 26 bytes | auditor's probe: 4000 × 4000 counts, 6 s; the new test hung before the fix | fixed: such records give empty rows, read as 0; test "reads records of no bytes as no kerning" |
| A3, crafted font | A `kern` subtable whose length is below its 6-byte header made the walk read the same subtable again, its kerning summed twice (−50 → −100); 65 535 subtables announced in 18 bytes took 10.8 s | auditor's probe; the new tests failed before the fix (one by a 13 s timeout) | fixed: such a table gives undefined, as a table passing its end does (`REF-OT-KERN`: the length includes the header); the count is bounded by what the table holds |
| A3, E1, damaged font | `unitsPerEm` outside 16 to 16 384 (`REF-OT-HEAD`) was read as is: 0 measured every text as Infinity or NaN; `numberOfHMetrics` 0 measured every text as 0 wide | new tests failed before the fix | fixed: `parseFixedMetrics` refuses both; `shapes.md` (Font metrics) says so |
| A2, playground test | `tool-placement.test.ts` expected 170.5 wide, true only at weight 200: happy-dom drops the `selected` weight (`.claude/rules/library.md`); the page's 400 gives 175.90625 | recomputed from `inter-metrics.json`: (11 351 − 93) × 32 / 2048 | fixed: weight 400 chosen in the test, `rotate(90 87.95313 58.07813)` |
| D1 | `Math.max(0, ...values)` spread by hand in `transpose` and `kerningOfCharacters`, beside `maximum`, made for it | `transpose.ts`, `kerning-of-characters.ts` | fixed: `maximum(…, 0)` |
| B1 | `DERIV-text-width` and `parseFontKerning` said browsers read the `kern` table; CSS Fonts 4 §6.12 only allows it | `REF-CSS-FONTS-4` re-read | fixed: "as CSS Fonts 4 allows a reader to do" |
| A1, E1 | `DERIV-text-width` step 1 presented `.notdef` as how a missing character is drawn; a CSS reader draws it with the next family that has it | `REF-CSS-FONTS-4` §2.1, read and quoted in the row | fixed: stated as a measuring convention |
| B1 | `REF-UNICODE-BLOCKS` quoted 8 of the 14 blocks measured | `measured-blocks.ts`; `Blocks.txt` re-read by the auditor | fixed: six ranges added |
| E1 | `textLines` claimed UAX #14 LB5, which also lists NL (U+0085) | `text-lines.ts` | fixed: "three of the breaks of LB5"; NL and U+2028/2029 asked at VAL-014 (below) |
| E1 | TSDoc of `letterSpacingCount` (characters, not typographic character units) and `parseXAdvance` (the file, not the `GPOS` table) | code | fixed |
| E3 | `docs/conventions/files.md` missed `src/io/text-style/`, `src/io/transform/`, `src/model/text-drawing/`, `playground/text/` | `ls` against the table | fixed |
| E, writing | `DERIV-bit-of-integer` cross-check omitted bit 4 of 149; "Indian" for "Indic"; `AGENTS.md` "Inter's shipped"; `REF-SVG2-STYLING` ".;"; the derivations index row of `DERIV-text-width` | doc-editor, math-expert | fixed |
| E4 | The improvement log still held the fallback family as open | ADR-0055 and Q31 agree with the code | fixed: row marked settled |

### Checks with evidence

| Check | Result | Evidence | Action |
| --- | --- | --- | --- |
| A1, A2 | every derivation step and every expected value recomputed | "Hello World" Arial 16: 10 584 × 16 / 2048 = 82.6875 (AC3); kerned 82.3984375; Inter 11 207 → 87.5546875; baselines, offsets, decorations, turn centers; cmap 4 spec example, wraparound 65 533 | none |
| A4, mutations | 89 mutations over the feature's 112 source files: 86 caught, 3 survived, each equivalent — `parseRecordPlaces` `>= 12` → `> 12` (a 12-byte file has no table either way), `populationCount` `length − 1` → `+ 1` (the bits past a number's length are 0), the name "Latin-1 Supplement" of `MEASURED_BLOCKS` (read by nothing); the 35 files left hold only types, constants checked through their users, or the path-data writers moved from E01 | scripted in a copy of the repository: operators first, then literals, then 12 written by hand (e.g. `isWordSeparator` without the no-break space, `measuredCharacters` in NFD, `textFontMetrics` without the fallback): each file's related tests run (`vitest related`), the file restored | none |
| A5, properties | every new `math` function has `test.prop` (`integerRange`, `isBitSet`, `isInIntegerRange`, `isSpanWithin`, `maximum`, `populationCount`, `transpose`); no `geometry` function added | verifier | none |
| A, precision | no defect: integer sums exact below 2⁵³, one multiplication and one division, baselines not accumulated, `toFixed` ties symmetric | math-expert | none |
| B1 | the 53 URLs of the new reference rows reachable (HTTP 200, 2026-10-15); `REF-OT-*`, `REF-CSS-FONTS-4`, `REF-CSS-TEXT-3`, `REF-SVG2-TEXT`, `REF-CSSWG-10193`, `Blocks.txt` re-read | curl; auditor and math-expert reading in the library | none |
| B2 | the weekly link check has not run on this branch; lychee absent locally | `gh run list` empty; the URLs above checked instead | none |
| B3 | no `@see` to an `[unverified]` row; no bypassing `eslint-disable` | lint; grep: four `eslint-disable` in `playground/text`, each justified | none |
| C | written from the specifications' field names; no opentype.js, fontkit or HarfBuzz structure; TSDoc formulas match their bodies; Inter (OFL) with its notice, licence file and `dependencies.md` row | auditor | none |
| D2, D3 | no URL under two IDs; no copied test hiding a case | `tests/references.test.ts`; verifier | none |
| D4, ADR-0049 | unnamed calculations: the index in a range (four places), a row-major address, whole records that fit (two places), a bit length, the sentinel −1 of `lineKerning` | auditor, math-expert | NAM-001 (improvement log), with the integer-range duplicates already logged |
| E2 | Q31 answered (ADR-0052, ADR-0055); Q36 provisional, to confirm at VAL-014 | `docs/domain/README.md` | none |
| E4 | SP-015, EN-031, EN-032, US-031, CHK-014, US-032 done; VAL-014 ready | feature file | none |
| F2 | no `.skip` or `.only`; two `it.skipIf` for Arial, absent outside macOS | grep | RET-003: a redistributable font as a fixture (improvement log) |
| F3 | demonstrable: the Text tool of the playground and the tagged tests `[F05.AC1]` to `[F05.AC6]` | `playground/text/*.test.ts` | VAL-014 |
| F4 | `AGENTS.md` and `CLAUDE.md` 147 lines together; rules still true | `wc -l` | none |

### Recorded, not fixed

| Finding | Why not now | Where |
| --- | --- | --- |
| GPOS read for the common case only: the second glyph's ValueRecord ignored and not consumed when `valueFormat2` is set; a second class past `class2Count` matching with 0; the LookupFlag (ignore marks) not read; the `kern` feature read for every script at once; a `kern` subtable over 65 535 bytes | Inter and Arial unaffected; each changes how arbitrary fonts are read, the subject of F15 | improvement log, F15's spike |
| A crafted `hhea` with ascender − descender + line gap ≤ 0 gives a block of height ≤ 0 | a business choice (below) | VAL-014 |
| The tests reading a real font run only on macOS | a font file to add to the repository | improvement log |
| Documentation restructuring proposed by the doc-editor (tables, shorter status lines, ADR-0054's tab claim) | style, no rule changed; ADR-0054 accepted, amended only by a new ADR | improvement log, RET-003 |

### Questions for the Product Owner (VAL-014)

1. **Where the line gap sits.** The library adds the font's line gap after each line: the first baseline is at the top plus the ascender, and the block's height includes a gap after the last line (Arial 16: 18.3984375 per line, of which 0.5234375 of gap). Options: keep it after each line; split it half above, half below each line; leave it out after the last line.
2. **Line breaks.** CR LF, CR and LF break a line; NL (U+0085), the line separator (U+2028) and the paragraph separator (U+2029) do not, and are measured as a missing glyph. Options: keep the three breaks; add NL; add all three.
3. **A font whose line height is zero or negative** (a crafted or damaged `hhea`). Options: refuse its metrics; keep the line height at 0; keep it as read.
4. **Kerning across scripts.** The `kern` feature is read for every script of the font at once. Options: keep the union; read the default and Latin scripts only.

### G. Next feature (F14, a box of text)

Sources and questions for its spike: line breaking opportunities (`REF-UAX14`, beyond LB5), CSS `overflow-wrap` and `word-break` for a word too long for its box (`REF-CSS-TEXT-3`), `text-overflow: ellipsis` and `line-clamp` (CSS Overflow 3 and 4), `text-wrap: balance` and `pretty` (CSS Text 4), vertical alignment in a box, `text-box-trim` and the cap height (`OS/2` `sCapHeight`, CSS Inline 3), a background drawn as a `<path>`; the questions above on the line gap and line breaks.

### H. Light evolvability check

| Question | Answer |
| --- | --- |
| Size | `src/` 9 397 lines; the longest modules are the font readers (`parse-kern-table.ts` 208 lines, `parse-pair-lookup.ts` 190), within the limits of their kind |
| Tools | `size-limit` due before the first npm publication (Inter's metrics 1.60 MB, already logged); Stryker still blocked by its issue 6210 (open, 2026-10-15) |
| Fitness functions | a fuzz property over crafted font tables (every field at its extremes) would have found the three crafted-font findings; candidate for F15 |
| Patterns | "index in a range" written four times (NAM-001) |
| Agent configuration | four parallel reviews found what three per-story reviews had not: the feature-level view pays |
| Process | the scripted mutation pass is a candidate agent tool (improvement log) |
