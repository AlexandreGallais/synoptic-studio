---
id: US-058
epic: E15
feature: F17
title: The ellipsis on the last line shown holding text
status: done
points: 1
---

# E15 · F17 · US-058 — The ellipsis on the last line shown holding text

As a symbol designer, I want the ellipsis of a cut text to end its last line holding text, so that "…" never stands alone on an empty line.

From the Product Owner's second pass of VAL-018 (2026-10-16), finding F4: a text of 4 lines, the 3rd empty, with at most 3 lines, showed "…" alone on the empty 3rd line. The cause: `shownLines` ended the last line shown with the ellipsis whatever it held, and the ellipsis cut of an empty line keeps nothing before "…" (`DERIV-text-box`, step 6).

**Decision** (provisional, the agent's proposal, to confirm at VAL-018; `docs/domain/shapes.md`, Text, Box): when a text is cut, by its maximum number of lines or by its box's height, with the ellipsis on, the ellipsis ends the last line shown that holds text, and the empty lines shown after it are dropped: they are not drawn and take no height in the block, so that a block in the middle or at the bottom of the box is placed by the lines left. A line of spaces is empty, its spaces trimmed as at the end of every line (`DERIV-text-box`, step 2). This departs from CSS, whose `block-ellipsis` goes on the last line box whatever it holds (`REF-CSS-OVERFLOW-4`, §4.2). With the ellipsis off, or when only empty lines are left out, the lines shown are unchanged. When no line shown holds text, the ellipsis stays on the last line shown.

## Acceptance criteria (F17.AC16)

- Given "Hello", "World", an empty line and "Hello" in Arial 16 in a box 60 wide, with at most 3 lines, when it is drawn, then it shows "Hello" over "World…" (57.78125 wide), its block 36.796875 tall: two lines.
- Given the same text in a box 60 tall with no maximum, three lines fitting (55.19531 ≤ 60), then it shows "Hello" over "World…" as well.
- Given a 3rd line of three spaces instead of an empty one, then it shows the same.
- Given the ellipsis off, then it shows "Hello", "World" and the empty line, unchanged.
- Given "Hello" and three line breaks with at most 3 lines, nothing but empty lines left out, then it shows "Hello" and two empty lines, with no ellipsis.

## Product Owner test

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Text tool, Box, width 200, height 100; content "A", "B", "", "C" | four lines, the 3rd empty |
| 2 | Max lines 3 | "A" over "B…"; no "…" alone on an empty line |
| 3 | Vertical alignment Bottom | "B…" ends at the box's bottom: the empty line takes no height |
| 4 | Max lines 0, then height 60 (three lines fit) | "A" over "B…" again, the cut made by the box's height |
| 5 | Ellipsis off | "A", "B" and the empty line, with no "…" |

## Tasks

One task = one commit, referenced as `US-058.Tn`.

- [x] T1 — The ellipsis on the last line shown holding text, the empty lines after it dropped; `DERIV-text-box` step 6, `shapes.md`, the checklist line (1 h)
- [x] T2 — A property of `shownLines`: of the lines the box shows, only empty ones are dropped, and each line kept is its wrapped line or a start of it ended with the ellipsis (0.5 h)
