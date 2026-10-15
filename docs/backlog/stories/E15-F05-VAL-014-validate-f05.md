---
id: VAL-014
epic: E15
feature: F05
title: Validate F05
status: ready
points: 1
---

# E15 · F05 · VAL-014 — Validate F05

The Product Owner tries the text in the playground, guided by short steps, and says what they see (ADR-0020).

## Acceptance criteria

- Given the playground, when the Product Owner follows the guided steps, then each option of the text is tried and the agent's decisions are confirmed or changed.

## Tasks

One task = one commit, referenced as `VAL-014.Tn`.

- [x] T1 — The guided test, tested end to end (1 h)
- [ ] T2 — The demo page (30 min)
- [ ] T3 — The Product Owner's validation (1 h)

## Guided test (T1)

35 steps at `http://localhost:5173`, also after E14's at `?guide=epic`: one line to do, one to look at, Next typing the values. `playground/guided-steps/epic-15/f05-steps.test.ts` checks each step end to end.

| Steps | What                                                                                                                                                                                                         |
| ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1     | Inter 4.1 installed, else the frame does not hug the letters                                                                                                                                                 |
| 2–14  | every field of the Text tool: content and lines, empty text, size, weight, italic, color and opacity, letter and word spacing, underline and strike-through, the ten cases, alignment, right to left, Hebrew |
| 15–20 | position, turned by 90, the angle kept from 0 to 359, the dashed frame, the turn on the group, values refused                                                                                                |
| 21    | each tool keeps its placement                                                                                                                                                                                |
| 22–32 | the decisions and questions below, one step each with its example                                                                                                                                            |
| 33–34 | in Chrome, Firefox and Safari: letter spacing after the last letter, our underline against the native one                                                                                                    |
| 35    | back to the rectangle, drawn as before (F05.AC6)                                                                                                                                                             |

A step of a text carries the Text tool's fields (`GuidedText`), which Next types. The guided steps are now one folder per epic (`epic-1/`, `epic-14/`, `epic-15/`), their type in `guided-step/` and the two lists in `lists/`, so that no folder passes 15 modules (ADR-0043).

## Decisions to confirm

Asked after the walk-through, in one questionnaire. Each is built as option 1; its step of the guided test shows it.

| #   | Decision or question                                                       | Example                                                                           | Options                                                                                                                                                                        | Step |
| --- | -------------------------------------------------------------------------- | --------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---- |
| 1   | Case transforms (Q36)                                                      | "pump P-101 is running": title `Pump P-101 Is Running`, camel `pumpP101IsRunning` | 1. as `shapes.md` says; 2. title case also after a hyphen or an underscore (`pump-on` gives `Pump-On`); 3. other                                                               | 11   |
| 2   | Left, center and right in a right-to-left text                             | "Running", 123.61 wide, under "Pump P-101", 175.91, aligned left                  | 1. the physical side, as CSS `left`: "Running" from 0 to 123.61; 2. the start of the line, as CSS `start`: from 52.30 to 175.91                                                | 13   |
| 3   | The bold button                                                            | weights 800 and 600                                                               | 1. pressed from 700, a click gives 400 when pressed, 700 otherwise (800 → 400, 600 → 700); 2. pressed at 700 only; 3. a click goes back to the weight before (800 → 700 → 800) | 24   |
| 4   | Bounds of letter and word spacing                                          | Inter 32                                                                          | 1. −100 000 to 100 000, the size's bound; 2. from minus the size to the size (−32 to 32); 3. −1 000 to 1 000                                                                   | 25   |
| 5   | A line tightened past nothing                                              | letter spacing −20: "Pump P-101" −4.09 wide, "Running" 3.61, centered             | 1. kept negative, placed by the same rule: it starts at 3.85, outside a block 3.61 wide; 2. counted as 0 wide: it starts at 1.80; 3. such a spacing refused                    | 26   |
| 6   | No stroke under a line 0 wide or less, nor one the font gives no thickness | the same text, underlined                                                         | 1. no stroke: one stroke only, under "Running"; 2. a stroke as long as the line's absolute width, 4.09 under "Pump P-101"                                                      | 27   |
| 7   | Line breaks, and AUD-015's question 2                                      | "Pump", U+2028, "P-101": one line 187.91 wide, the separator a 21 box             | 1. CR LF, CR and LF only; 2. also NL (U+0085); 3. also NL, the line separator U+2028 and the paragraph separator U+2029: two lines                                             | 28   |
| 8   | A font's name                                                              | `" Inter"`, `"Inter⏎Bold"`                                                        | 1. stored trimmed (`Inter`), a control character refused; 2. stored as typed; 3. trimmed, a control character turned into a space (`Inter Bold`)                               | 23   |
| 9   | Characters measured                                                        | € − → ≤ ™ ș ț Ω Ж each in its width, 中 a 21 box                                  | 1. Latin, Greek, Cyrillic, Hebrew and the symbols labels use (2 080 code points); 2. also box drawing, geometric shapes (■ ▲ ●) and superscripts (² ³); 3. Latin only          | 29   |
| 10  | A font without metrics (ADR-0055)                                          | Font Arial, not read yet                                                          | 1. measured with its fallback's metrics, else Inter's: 175.91 wide; 2. refused until its file is read; 3. measured with its fallback only, nothing when that one is unread     | 22   |
| 11  | The turn written on the group (ADR-0055)                                   | angle 90, underlined and struck through                                           | 1. one `rotate(90 87.95 38.72)` on the `<g>` of the text and its 4 strokes; 2. the same `transform` on the `<text>` and on each stroke                                         | 19   |
| 12  | Where the line gap sits (AUD-015, question 1)                              | Arial 16: 18.40 per line, of which 0.52 of gap; two lines                         | 1. after each line, the last included: 36.80 tall; 2. half above, half below each line: 36.80 tall, the first baseline 0.26 lower; 3. not after the last line: 36.27 tall      | 30   |
| 13  | A font whose line height is 0 or negative (AUD-015, question 3)            | a damaged `hhea`: ascender − descender + line gap = 0                             | 1. kept as read, the lines on top of each other; 2. its metrics refused; 3. its line height kept at 0                                                                          | 31   |
| 14  | Kerning across scripts (AUD-015, question 4)                               | "To" in Inter 32: 2.5 closer than without kerning (−160 units)                    | 1. the `kern` feature of every script at once; 2. the default and Latin scripts only                                                                                           | 32   |
| 15  | The dashed frame as a text's selection                                     | the text turned by 30                                                             | 1. dashed, as a shape's straight box; 2. a solid thin frame; 3. other                                                                                                          | 18   |
| 16  | Letter spacing after the last letter (measured in three browsers)          | "HH" Inter 16, letter spacing 10                                                  | 1. between the letters only, 33.78125, as built; 2. after the last letter too, 43.78125, as a browser that adds it (`REF-CSSWG-10193`)                                         | 33   |
| 17  | The underline's position (measured in three browsers)                      | "HH" Inter 200: our stroke's top at 227.73, 13.67 thick                           | 1. `post`'s position read as the top of the stroke, as built; 2. read as its middle: 6.84 lower                                                                                | 34   |
