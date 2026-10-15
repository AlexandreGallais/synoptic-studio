---
id: EN-036
epic: E15
feature: F14
title: A balanced wrap that evens the lines' lengths
status: ready
points: 2
---

# E15 · F14 · EN-036 — A balanced wrap that evens the lines' lengths

With the balanced wrap style, the library keeps the number of lines of the greedy wrap and chooses the breaks that leave the least sum of the squares of the room left in each line (`DERIV-text-box`, step 9).

## Acceptance criteria

- Given "Pump P-101 is running" in Arial 16, kerning left out, in a box 110 wide and tall enough for every line, when balanced, then the lines are "Pump P-101" (88.9375) and "is running" (69.375), of cost `21.0625² + 40.625²` = 2094.01953, instead of the greedy "Pump P-101 is" (104.9375) and "running" (53.375), of cost 3232.01953 (`REF-CSS-TEXT-4`, §5.4).
- Given "AAA BB CC DDDDD" in a font whose every character is 1 wide, in a box 6 wide, when balanced, then the lines are "AAA", "BB CC", "DDDDD", of cost 11, instead of the greedy 17 (`REF-WIKI-KNUTH-PLASS`).
- Given "Hello World Hello" in a box 90 wide, when balanced, then the lines stay "Hello World" and "Hello": the other cut costs the same, and the later break is kept.
- Given two paragraphs, when balanced, then each is balanced on its own; given a paragraph holding a word too long for the box, or a text cut by the ellipsis, then its lines stay those of the greedy wrap (provisional, VAL-015).
- Given any text and box, when balanced, then the number of lines equals the greedy wrap's and the cost is no more than the greedy's (a property test).

## Tasks

One task = one commit, referenced as `EN-036.Tn`.

- [x] T1 — The least raggedness with the greedy number of lines, by dynamic programming over the breaks, `@see DERIV-text-box`, `REF-WIKI-KNUTH-PLASS`; tests first, the two properties above (3 h)
