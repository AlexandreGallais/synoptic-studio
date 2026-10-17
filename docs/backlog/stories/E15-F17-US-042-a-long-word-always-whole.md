---
id: US-042
epic: E15
feature: F17
title: A word too long always stays whole
status: done
points: 2
---

# E15 · F17 · US-042 — A word too long always stays whole

As a symbol designer, I want a word too long for its box to stay whole so that a text has one fewer option and never breaks a word in the middle.

From the Product Owner's validation of E15 (2026-10-15): the choice between breaking a word at a character and moving it whole is removed (VAL-015, decisions 3 and 4; `shapes.md`, Text, Box). Changes F14's criterion 1.

## Acceptance criteria

- Given "running" (53.4 wide in Arial 16) in a box 50 wide, the ellipsis off, when it is drawn, then it shows one line "running", whole, overflowing the box by 3.4.
- Given the same text with the ellipsis on, when it is drawn, then it shows "runn…".
- Given "Hello World" in a box 50 wide, when it is drawn, then it shows "Hello" over "World", as before.
- Given the Text tool, when a box is chosen, then no option for a word too long is offered; F14's criterion 1 says that a word too long stays whole.

## Product Owner test

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Text "running", Arial 16, in a box 50 × 40, the ellipsis off | "running" whole, a little past the box |
| 2 | Turn the ellipsis on | "runn…" |
| 3 | Look for the option of a word too long | none |

## Tasks

One task = one commit, referenced as `US-042.Tn`.

- [x] T1 — The library keeps every word whole, the option and the break at a character removed, tests updated (3 h)
- [x] T2 — The playground's option and its guided steps removed; `shapes.md` and F14's criterion 1 updated (2 h)
