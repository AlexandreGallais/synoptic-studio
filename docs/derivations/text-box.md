# DERIV-text-box — Lines wrapped in a box, cut with an ellipsis, placed by a line height, a trim and a vertical alignment

Used by: F14 (EN-033 to EN-036, US-033, US-034), F17 (US-055, step 11). Builds on `DERIV-text-width`, which measures a line and places it across its block. Sources:

| Source | For |
| --- | --- |
| `REF-UAX14` | break opportunities after spaces, spaces not measured for fit |
| `REF-CSS-TEXT-3` | soft wrap opportunities, hanging spaces, `overflow-wrap` |
| `REF-WIKI-LINE-WRAP` | the greedy wrap |
| `REF-WIKI-KNUTH-PLASS` | minimum raggedness by dynamic programming |
| `REF-WIKI-PREFIX-SUM` | the scan of the wrap's steps |
| `REF-CSS-TEXT-4` | `text-wrap-style: balance` |
| `REF-CSS-OVERFLOW-3` | `text-overflow: ellipsis` |
| `REF-CSS-OVERFLOW-4` | `line-clamp`, `max-lines`, `block-ellipsis` |
| `REF-CSS-INLINE-3` | `line-height`, half-leading, cap height, `text-box-trim`, content area |
| `REF-OT-OS2` | `sCapHeight` |
| `REF-OT-HHEA` | ascender, descender, line gap |
| `REF-SVG2-PATHS` | sub-paths of one `<path>` |
| `REF-SVG2-FILL-RULE` | overlaps filled once |
| `REF-ECMA-262-MATH-CEIL` | the ceiling of a number |

## Notation

| Symbol | Definition |
| --- | --- |
| `W`, `H` | the box's width and height, integers in user units |
| `w(t)` | the width of the string `t` measured as one line (`DERIV-text-width`, steps 1 to 3) |
| `trim(t)` | `t` without the spaces U+0020 at its end |
| `s`, `U` | the text's size and its font's units per em |
| `A, D, G` | the `hhea` ascender, descender (negative) and line gap, in font units |
| `C` | the cap height `sCapHeight` of `OS/2`, in font units; `0.66 × U` when the table is older than version 2, too short to hold the field, or gives 0 or less |
| `λ` | the line height in user units: the font's `(A − D + G) × s / U` when automatic, otherwise the integer chosen |
| `N` | the maximum number of lines, 0 for none |
| `L` | the number of lines the wrap gives (steps 3 and 4), 0 for an empty text |
| `v` | the vertical alignment: 0 top, ½ middle, 1 bottom |
| `…` | the ellipsis, U+2026 |

## Step 1 — Paragraphs and where a line may wrap

The content is cut into paragraphs at each line break typed (`DERIV-text-width`; `REF-UAX14`, LB5); each paragraph is wrapped on its own. Inside a paragraph, only spaces open a break (the scope of F14): a break is allowed after the last space of each run of spaces U+0020, "the locally most optimal one" (`REF-UAX14`, SP and LB18), and never after a no-break space U+00A0 (`REF-UAX14`, LB12, class GL). The paragraph is therefore a list of **tokens**, each a run of characters other than U+0020 followed by the spaces after it. Spaces at the start of a paragraph join its first token, so that no line is made of them alone: a design choice, not a rule of the source, since LB18 would allow a break after them (confirmed at VAL-015, 2026-10-15).

## Step 2 — A line's width for fit, and its spaces at a wrap

"Spaces at the end of a line are ordinarily not measured for fit" and are "kept together invisibly on the previous line" (`REF-UAX14`, SP); CSS makes them hang, "not considered when placing the rest of the line during text alignment" (`REF-CSS-TEXT-3`, §4.1.2). So a candidate line `t` fits when:

```text
w(trim(t)) ≤ W
```

and a line ended by a wrap is drawn and measured as `trim(t)`. The last line of a paragraph in a box is drawn and measured as `trim(t)` too: its trailing spaces hang as every line's, "not considered when placing the rest of the line during text alignment" (`REF-CSS-TEXT-3`, §4.1.2), so they neither place the line nor lengthen its background (step 10); every line the greedy wrap makes fits but one of a long word, which stands whole on its own line, overflowing (US-042).

In Arial 16 in a box 50 wide, "Hello World " gives "World", 41.78125 wide; right-aligned it sits at 50 − 41.78125 = 8.21875, not at 3.7734375, a space's width (4.4453125) from the edge. Decided on 2026-10-16 for US-049, provisional until VAL-018; before it, the last line kept the spaces typed for its placement, as CSS `pre-wrap` does before a forced break, and its band ran past its last word. A text that fits its content is not wrapped and keeps its lines as typed (F05).

## Step 3 — The greedy wrap

"A greedy algorithm that puts as many words on a line as possible" uses "the minimum possible number of lines" (`REF-WIKI-LINE-WRAP`), and the user agent "must minimize the amount of content overflowing a line by wrapping the line at a soft wrap opportunity" (`REF-CSS-TEXT-3`, §5). For each token in order: on an empty line it is placed; otherwise it joins the line when the line with it fits (step 2), and starts a new line when it does not.

## Step 4 — A word too long for the box

> [!NOTE]
> The choice below was removed by the Product Owner at VAL-015 (2026-10-15): a long word always moves whole. US-042 removed the break at a character from the library; it is kept here as a record, with the examples below that break at a character.

A token that does not fit on a line of its own is a long word. The designer chooses (F05 refinement, round 2):

- **break at a character**, as `overflow-wrap: anywhere`: the word "may be broken at an arbitrary point if there are no otherwise-acceptable break points in the line", grapheme clusters kept together, "No hyphenation character is inserted" (`REF-CSS-TEXT-3`, §5.4):
  - the word first starts a new line, as every token that does not fit; then its units (a character and the nonspacing marks after it, `DERIV-text-width` step 3) join the line one by one;
  - it is cut before the first unit that does not fit, at least one unit other than the spaces that start the line kept; the rest is a new token on the next line, again;
  - with widths that grow at each unit, this is the longest run that fits; a negative spacing may narrow a longer run again, which the cut does not look for;
  - a no-break space U+00A0 stays with the unit after it, so that a word is never cut right after one (LB12 applied to the cut too; confirmed at VAL-015, 2026-10-15);
- **move whole**, as `overflow-wrap: normal`: "Lines may break only at allowed break points" (`REF-CSS-TEXT-3`, §5.4), so the word stays whole on its own line and overflows the box:
  - "a single word is too long to fit" is a case of text overflowing a line (`REF-CSS-OVERFLOW-3`, §5.1), so with the ellipsis on that line is cut with an ellipsis (step 6), and with it off the word is drawn whole, overflowing.

## Step 5 — The number of lines shown

A block of `n` lines is `h(n) = n × λ` tall, or `(n − 1) × λ + C × s / U` when trimmed (step 7). The lines shown are the most whose block fits in the box, at least one, at most `N` when a maximum is set, and never more than the `L` lines the wrap gives:

```text
n = min( max(1, max{ n ≥ 1 : h(n) ≤ H }), N or ∞, L )     (N = 0: no maximum; max ∅ = 0)
```

An empty text (`L = 0`) shows no line and draws nothing (F05).

`line-clamp` limits a block "to the specified number of lines; remaining content is fragmented away and neither rendered nor measured", and `max-lines` takes only positive integers (`REF-CSS-OVERFLOW-4`, §5.1, §5.2). At least one line is shown even when the box is shorter than one line, so that a text never vanishes (provisional rule, VAL-015).

## Step 6 — The ellipsis

The ellipsis is an option of the text, on unless the designer turns it off: F05's refinement says a text too long "can end with an ellipsis" (round 1). With it off, a cut text simply stops at the last line shown, and a word moved whole is drawn whole, overflowing. With it on, the last line shown ends with an ellipsis when lines other than empty ones were left out (`block-ellipsis: auto` renders U+2026 "at the end of the affected line box", `REF-CSS-OVERFLOW-4`, §4.2; empty lines left out, as after a content ending with a line break, hide nothing and add none, confirmed at VAL-015, 2026-10-15), and so does a line wider than the box (`text-overflow: ellipsis`, `REF-CSS-OVERFLOW-3`, §5.1) when its cut removes at least one unit. In both cases the ellipsis is measured with the rest of the line, and "implementations must hide characters … at the end edge of the line as necessary to fit the ellipsis" (`REF-CSS-OVERFLOW-3`):

```text
t ← trim(line)
while t holds more than its leading spaces and no-break spaces and one unit, and (w(t + "…") > W or t ends with U+00A0):
    t ← trim(t without its last unit)
line ← t + "…"
```

The line of a text cut that takes the ellipsis is its last line shown that **holds text**, and the empty lines shown after it are dropped, neither drawn nor counted in the block's height (step 5's `h(n)` counts the lines left): with `k` lines shown and `e` the index of the last non-empty one, the lines shown become the first `e + 1`, the last of them ellipsed as above; with none non-empty, the ellipsis stays on the `k`-th. A line of spaces is empty: the spaces ending a line are dropped (step 2, `trim(t)`).

CSS places the block ellipsis "at the end of the affected line box" whatever it holds (`REF-CSS-OVERFLOW-4`, §4.2), so an empty last line would show "…" alone; the library departs from it so that the ellipsis follows the text it shortens (agent's proposal at VAL-018's second pass, 2026-10-16, provisional, US-058). "Hello", "World", "" and "Hello" in Arial 16 in a box 60 wide with at most 3 lines shows "Hello" over "World…" ((5348 + 2048) / 128 = 57.78125 ≤ 60), a block 2 × 18.3984375 = 36.796875 tall.

"The first character … on a line must be clipped rather than ellipsed" (`REF-CSS-OVERFLOW-3`): the first unit other than a space always stays — spaces and no-break spaces at the start of a line never count as the unit kept. So a line wider than the box whose cut would remove nothing, one character wide, keeps no ellipsis: "H" alone in a box 5 wide shows "H", overflowing, with no ellipsis (confirmed at VAL-015, 2026-10-15, then with the break at a character, removed by US-042). A line of no-break spaces only keeps none of them before the ellipsis. The last line of a text cut always ends with one ("H…").

A cut never comes right after a no-break space U+00A0, which stays with the unit after it, as at any break (`REF-UAX14`, LB12; confirmed at VAL-015, 2026-10-15): "Hello", a no-break space and "World" in a box 70 wide ends "Hello…", 52.46094 wide, not "Hello" and the no-break space before the ellipsis.

CSS goes on: "If there is insufficient space for the ellipsis, then clip the rendering of the ellipsis itself" (`REF-CSS-OVERFLOW-3`). Clipping needs a clip path, an SVG primitive the project does not allow (ADR-0052, decision 7), so the library departs from CSS here: a box narrower than its first character and the ellipsis shows both, overflowing the box (confirmed at VAL-015, 2026-10-15; the alternatives are to draw only what fits, or nothing). A face without a glyph for U+2026 writes three full stops, "three dots \"...\" if the ellipsis character is unavailable" (`REF-CSS-OVERFLOW-3`).

CSS Overflow 4 would place the block ellipsis "after the last soft wrap opportunity" that lets it fit, a whole word removed (`REF-CSS-OVERFLOW-4`, §4.2); a cut at a unit keeps more of the text in a narrow box, and is the rule for both cases here (confirmed at VAL-015, 2026-10-15).

## Step 7 — Line height, baselines and the trim

When the line height is automatic, the lines follow `DERIV-text-width` step 4: `λ = (A − D + G) × s / U`, and the first baseline lies `A × s / U` below the block's top, as in F05.

When a line height `λ` is chosen, CSS computes "the leading L as L = line-height − (A + D)", with `A` and `D` the ascent and the depth below the baseline, and adds "half the leading … above A … and the other half below D", `L` possibly negative (`REF-CSS-INLINE-3`, §5.3). With the descender `D` negative:

```text
first baseline below the block's top = (λ − (A − D) × s / U) / 2 + A × s / U
```

When the text is trimmed, its first line's top is the cap height and its last line's bottom the baseline (`text-box-trim` with the edges cap height and alphabetic; the cap height is "Calculated using `sCapHeight` in OpenType", `REF-CSS-INLINE-3`, §3.2, §6, §6.2). The first baseline then lies `C × s / U` below the block's top, whatever `λ`, and the block of `n` lines is `(n − 1) × λ + C × s / U` tall. `sCapHeight` exists from version 2 of `OS/2` and is set to 0 when the font has no `H` (`REF-OT-OS2`); then the cap height is `0.66 em`, the fallback `REF-CSS-INLINE-3` suggests (Appendix A.2). In every case:

```text
baseline of line k (from 0) = top of the block + first baseline + k × λ
```

## Step 8 — The block placed in the box

A text that fits its content has its block's top at its `y` and is as wide as its widest line (`DERIV-text-width`). In a box, the lines are placed across the box's width `W` by their alignment (`DERIV-text-width`, step 5, with `B = W`), and the block of `n` lines is placed down the box by the room it leaves, as a line is placed across:

```text
top of the block = y + (H − h(n)) × v,   v = 0 (top), ½ (middle), 1 (bottom)
```

A block taller than the box (one line in a box shorter than it) has negative room: centered, it overflows both edges equally. A text in a box turns about the middle of its box (`DERIV-turned-shape`, step 4; F05 refinement, round 2): `(x + W / 2, y + H / 2)`.

## Step 9 — The balanced wrap

> [!NOTE]
> The balanced wrap was removed by the Product Owner at VAL-015 (2026-10-15): lines always wrap automatically. US-043 removed it from the library; this step is kept as a record, with the balanced examples below.

With `balance`, "line breaks are chosen to balance the remaining (empty) space in each line box", without changing the number of lines (for 5 or fewer, "must not"); balance is measured by "the standard deviation from the average inline-size of the remaining space in each line box … (including lines that end in a forced line break)", each paragraph "processed separately", and "the exact algorithm is UA-defined" (`REF-CSS-TEXT-4`, §5.4).

For a paragraph of `n` greedy lines, the `n` lines are chosen to minimize the sum of the squares of the space each leaves, every line growing while it fits:

```text
minimize  Σᵢ₌₁ⁿ (W − wᵢ)²    over the ways to cut the tokens into n lines
                             whose every line, and every first part of it, fits: w ≤ W
```

A line that grows while it fits is one the greedy wrap could make from its start: from token `i`, it takes token after token and closes at the first that does not fit. When each token widens the line, which holds for spacing 0 or more and kerning smaller than the advance it follows, every line that fits grows while it fits, and the minimum is the least over every cut into `n` lines that fit. A negative letter or word spacing may narrow a longer line again, and a line that fits only past a first part that does not is left out: the result is then the least among the cuts whose lines grow while they fit (confirmed at VAL-015, 2026-10-15).

This is the minimum raggedness of TeX, which "minimizes the sum of the squares of the lengths of the spaces at the end of lines" (`REF-WIKI-LINE-WRAP`), computed by dynamic programming over the break points (`REF-WIKI-KNUTH-PLASS`): the least cost of the first `j` tokens on `k` lines is the least, over `i < j`, of the cost of the first `i` tokens on `k − 1` lines plus the square of the space left by tokens `i` to `j − 1` on one line. Since `Σ (W − wᵢ)` changes only by the spaces dropped at the breaks, minimizing the squares with `n` fixed tends to lower their spread, which CSS's standard deviation measures: a heuristic for CSS's aim, not a proof that the standard deviation is the least.

The greedy cut is one of the candidates (every greedy line fits, step 2), so the balanced cut has no greater sum of squares than it, and has the same `n` lines. On equal cost, the later break is kept, as the greedy wrap would choose. A paragraph holding a long word keeps its greedy lines. A text cut by its box or its maximum number of lines, the ellipsis on or off, keeps its greedy lines too: a departure from CSS, where "the clamping effect is applied first, then the remaining lines are balanced" (`REF-CSS-TEXT-4`, §5.4), chosen so that the lines shown are those the text would show without balance.

A paragraph is balanced only when its greedy wrap gives at most 10 lines, as "UAs may treat this value as auto if there are more than ten lines to balance" (`REF-CSS-TEXT-4`, §5.4), and when it holds at most 500 tokens, a bound of the library's own on the candidate lines, the algorithm being "UA-defined"; otherwise it keeps its greedy lines.

The candidate lines are those that grow while they fit, from each start, so every greedy line is one of them. Each token is measured once and the lines are measured by joining the measures (`DERIV-text-width`, step 2). The dynamic programming keeps, at each end `j`, only the numbers of lines `k` that a whole cut of `n` lines may give the first `j` tokens: `k` at least one more than the fewest at the starts of the lines ending at `j`, and at least `n − (T − j)` for `T` tokens, each token fitting alone; `k` at most `j`, and at most `n − g(j)`, `g(j)` the fewest lines that hold the tokens from `j` to the end, computed backwards the same way (`g(T) = 0`, `g(i) = 1 + min g(j)` over the lines from `i` to `j` that fit). A cell outside these bounds lies on no cut of `n` lines, so leaving it out changes no result.

Each end reads only the columns of the `m` ends before it, `m` the most tokens a line holds, and the steps are a scan (`REF-WIKI-PREFIX-SUM`) computed by halves, so that no result is copied once per token: the work is about `T × m` for the candidate lines, at most 500 × 500.

## Step 10 — The background behind each line

A highlighter covers each line's content area, "sized and positioned to fit … text from its first available font", for example from "the maximum ascender and descender of the font" (`REF-CSS-INLINE-3`, §6.4): from the ascender to the descender, as long as the line drawn (with its ellipsis). For a line of width `w > 0`:

```text
x = left of line,  y = baseline − A × s / U,  width = w,  height = (A − D) × s / U
```

A line `w ≤ 0` wide gets none (`textBackgrounds`). The rectangles are one `<path>`, one closed sub-path per line, each drawn with the rectangle tool's corners, sharp (`decorationCorners`), clockwise from its top-left corner (Q11), each starting with its own `moveto` (`REF-SVG2-PATHS`, §9.3.3; `formatPathData`), drawn before the `<text>` in the text's group so that the letters stay on top and turn with it.

The band ignores the line height and the trim. CSS §6.4 goes on "If text-box-trim indicates trimming, then the specified metric must be used" for an inline box's own content area; the trim here is the block's, which CSS applies to the block, not to each line's content area, so the band keeps the ascender and descender: a line height chosen spreads its leading around the content area (step 7), so the bands of two lines leave `λ − (A − D) × s / U` between them, or overlap by that much when it is negative; a trimmed block starts at the cap height, so the first band rises `(A − C) × s / U` above the block's top, and above the box's top in a box placed at the top.

Overlapping bands are filled once, the rectangles being sub-paths of one `<path>` turning the same way, filled by the nonzero rule (`REF-SVG2-FILL-RULE`); CSS paints each line's background on its own, so a see-through overlap would look darker there: a departure kept for one `<path>` (confirmed at VAL-015, 2026-10-15). With Arial 16, "Hello" over "World" at a line height of 24 gives bands from `3.0625` and `27.0625`, 6.125 apart; trimmed, at the automatic line height, from `11.4609375 − 14.484375 = −3.0234375` and `15.375`.

## Step 11 — The box around a measured block

A text that fits its content and is put in a box keeps its lines and its place when the box holds its block (US-055): each line then fits the box's width (step 2), the wrap leaves every paragraph on its line (step 3), and every line is shown (step 5). Both tests hold within a rounding error `ε = EPSILON` (`w(trim(t)) ≤ W + ε`, `h(n) ≤ H + ε`), and a box's sides are integers (ADR-0003). The smallest such box rounds each side of the block `w × h` up, "the smallest (closest to -∞) integral Number value that is not less than x" (`REF-ECMA-262-MATH-CEIL`), after taking off the error, so that a side that is an integer within it keeps it:

```text
W = ⌈w − ε⌉,   H = ⌈h − ε⌉
```

`W` is the least integer with `w ≤ W + ε`: `W ≥ w − ε` by the ceiling, and `W − 1 < w − ε`. The same holds for `H`. Lines are then aligned across `W` and the block placed down `H` (step 8), so a line moves across by at most `(W − w) × a` and the block down by at most `(H − h) × v`, both less than one unit, `a` the line's alignment (0 left, ½ center, 1 right). The block measures its lines with their trailing spaces when it fits its content (`DERIV-text-width`) and without them in a box (step 2), so the box is never too narrow, unless a negative word spacing makes a trailing space narrower than nothing: the line then wraps or overflows, as any line wider than its box.

## Cross-check

Arial 5.01 at size 16, kerning left out: `U = 2048`, `A = 1854`, `D = −434`, `G = 67` (`DERIV-text-width`); `OS/2` version 3, `sCapHeight = 1467`; advances read from the file by a script: `…` 2048, `.` 569, no-break space 569, `P` 1366, `u` 1139, `m` 1706, `p` 1139, `-` 682, `1` 1139, `0` 1139, `i` 455, `s` 1024, `n` 1139, `g` 1139, and those of `DERIV-text-width`. One font unit is `16 / 2048 = 1 / 128` user unit.

| Word or string | Units | Width |
| --- | --- | --- |
| `Hello` | 4 667 | 36.46094 |
| `World` | 5 348 | 41.78125 |
| `Pump` | 5 350 | 41.79688 |
| `P-101` | 5 465 | 42.69531 |
| `is` | 1 479 | 11.55469 |
| `running` | 6 832 | 53.375 |
| space | 569 | 4.44531 |
| `…` | 2 048 | 16 |
| `...` | 1 707 | 13.33594 |

## Checks (test cases)

| Case | Computation | Expected |
| --- | --- | --- |
| "Hello World", box 50 wide, H large | `4667 + 569 + 5348 = 10 584` units, 82.6875 > 50: wrap after the space | "Hello" (36.46094) / "World" (41.78125) |
| "Hello World", box 60 wide, H large, at most 1 line | "Hello" + "…" = `(4667 + 2048) / 128` = 52.46094 ≤ 60 | "Hello…", 52.46094 wide |
| "Hello World", box 50 wide, H large, at most 1 line | 52.46094 > 50; without `o`: `(4667 − 1139 + 2048) / 128` = 43.5625 ≤ 50 | "Hell…", 43.5625 wide |
| "Hello World Hello World", box 60, H large, at most 2 lines | four lines of one word; the second with "…": `(5348 + 2048) / 128` = 57.78125 ≤ 60 | "Hello" / "World…", 57.78125 wide |
| the same, box 60 × 40, no maximum | `h(2) = 36.79688 ≤ 40 < h(3) = 55.19531`; `L = 4` | 2 lines: "Hello" / "World…" |
| the same, box 60 × 10, no maximum | no `n ≥ 1` fits: `max ∅ = 0`, raised to 1; `L = 4` | 1 line: "Hello…", 52.46094 wide |
| (record, US-042) "Pump P-101 is running", box 50, break at a character | "running" 53.375 > 50 starts its line; "runnin" `5693 / 128` = 44.47656 ≤ 50, with `g` 53.375 | "Pump" / "P-101" / "is" / "runnin" / "g" |
| the same, move whole, ellipsis on | "running" overflows: `(6832 − 1139 − 1139 − 455 + 2048) / 128` = 48.02344 ≤ 50, one unit more 51.57813 | "Pump" / "P-101" / "is" / "runn…" |
| the same, move whole, at most 2 lines | "P-101…" `7513 / 128` = 58.69531 > 50; "P-10…" `6374 / 128` = 49.79688 | "Pump" / "P-10…" |
| "Hello World", box 10 wide, H large, at most 1 line | the first unit stays: `(1479 + 2048) / 128` | "H…", 27.55469 wide, overflowing |
| "Hello World", box 60 wide, H large, at most 1 line, ellipsis off | the text stops at the last line shown | "Hello", 36.46094 wide |
| "Hello World Hello World", box 60, H large, at most 2 lines, ellipsis off | the same | "Hello" / "World", 41.78125 wide |
| "Pump P-101 is running", box 50, H large, move whole, ellipsis off | the word drawn whole | "Pump" / "P-101" / "is" / "running", the last 53.375 wide, overflowing |
| two spaces then "Hello World", box 10 wide, H large, move whole, at most 1 line | the spaces are not the unit kept | the two spaces, "H" and "…" |
| (record, US-042) "Hello", break at a character, box 5 wide | each letter alone, "ll" 7.11 > 5; a cut would remove nothing | "H" / "e" / "l" / "l" / "o", no ellipsis |
| "Hello", a line break, box 60, at most 1 line | the line left out is empty | "Hello", no ellipsis |
| "Hello" / "World" / "" / "Hello", box 60, at most 3 lines | "Hello" left out; the third shown is empty: dropped, "World" ellipsed `(5348 + 2048) / 128` = 57.78125 | "Hello" / "World…", a block 36.796875 tall |
| the same, box 60 × 60, no maximum | `h(3) = 55.19531 ≤ 60 < h(4) = 73.59375`: 3 shown, the third empty | "Hello" / "World…" |
| the same, its third line three spaces | trimmed to "" (step 2) | "Hello" / "World…" |
| "Hello", a no-break space, "World", box 70 wide, at most 1 line | "Hello\u00A0W…" `9217 / 128` = 72.00781 > 70; "Hello\u00A0…" fits but ends with the no-break space | "Hello…", 52.46094 wide |
| an empty text in a box 60 × 60 | `L = 0` | no line, nothing drawn |
| ellipsis missing from the face, a line cut | three full stops: `3 × 569 / 128` | "...", 13.33594 wide |
| baselines of "Hello" / "World", automatic line height, H large | `λ = 2355 / 128` = 18.39844; first `1854 / 128` | 14.48438, 32.88281 |
| the same, line height 24 | `(24 − 2288 / 128) / 2 + 14.48438` = `3.0625 + 14.48438` | 17.54688, 41.54688; block 48 tall |
| the same, trimmed, automatic line height | first `1467 / 128`; block of 2 lines `18.39844 + 11.46094` | 11.46094, 29.85938; block 29.85938 tall |
| trimmed at size 32, H large | `1467 × 32 / 2048` | first baseline 22.92188 below the top |
| "Hello World", box 50 × 60, middle | `L = 2`, so `n = min(3, ∞, 2) = 2` although `h(3) = 55.19531 ≤ 60`; room `60 − 36.79688` = 23.20313, half before; baseline `11.60156 + 14.48438` | top 11.60156, first baseline 26.08594 |
| the same, bottom | all the room before: `23.20313 + 14.48438` | first baseline 37.6875 |
| the same, trimmed, middle | `(60 − 29.85938) / 2 + 11.46094` | first baseline 26.53125 |
| (record, US-043) "Pump P-101 is running", box 110, balanced | greedy "Pump P-101 is" (104.9375) / "running" (53.375): `5.0625² + 56.625²` = 3232.01953 | "Pump P-101" (88.9375) / "is running" (69.375): `21.0625² + 40.625²` = 2094.01953 |
| (record, US-043) "AAA BB CC DDDDD", each 1 wide, box 6, balanced | greedy "AAA BB" / "CC" / "DDDDD": `0² + 4² + 1²` = 17 (`REF-WIKI-KNUTH-PLASS`) | "AAA" / "BB CC" / "DDDDD": `3² + 1² + 1²` = 11 |
| (record, US-043) "Hello World Hello", box 90, balanced | "Hello World" / "Hello" and "Hello" / "World Hello" both cost `7.3125² + 53.53906²` | the later break: "Hello World" / "Hello" |
| background of "Hello" / "World" at (0, 0), fitting its content | `y = 14.48438 − 14.48438`, height `(1854 + 434) / 128` | x 0, y 0, 36.46094 × 17.875 |
| background of its second line "World" | `y = 32.88281 − 14.48438` | y 18.39844, 41.78125 × 17.875 |
| a box 60 × 40 at (0, 0), turned | the middle of the box | about (30, 20) |
| "Pump P-101", Inter 16, its block 87.953125 × 19.359375, put in a box (step 11) | `⌈87.953125 − ε⌉`, `⌈19.359375 − ε⌉` | a box 88 × 20 |
| a block 88.0000000001 × 20 put in a box | `88.0000000001 − 10⁻⁹ < 88` | a box 88 × 20 |
