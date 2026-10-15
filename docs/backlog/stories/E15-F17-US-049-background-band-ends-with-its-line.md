---
id: US-049
epic: E15
feature: F17
title: A wrapped text's background band ends with its line
status: ready
points: 2
---

# E15 · F17 · US-049 — A wrapped text's background band ends with its line

As a symbol designer, I want each line's background to end where the line ends so that a highlighted text in a box looks right.

From the Product Owner's validation of E15 (2026-10-15): the band behind a wrapped line may run past the line's end, to the right, on a new line. Reproduction and cause from SP-019.

## Acceptance criteria

- Given the case SP-019 reproduces (a wrapped text with a background), when it is drawn, then each band runs from the line's start to its end, its ellipsis included, never past it.
- Given "Hello World" in Arial 16 in a box 50 wide with a background, when drawn, then the band of "Hello" is as long as "Hello" and that of "World" as long as "World".
- Given a text fitting its content with a background, when drawn, then its bands are as before (F14.AC4).

## Product Owner test

| Step | Do                                                       | You should see                      |
| ---- | -------------------------------------------------------- | ----------------------------------- |
| 1    | The case of SP-019, background on                        | each band ending with its line      |
| 2    | "Hello World", Arial 16, in a box 50 × 40, background on | two bands, each as long as its word |

## Tasks

One task = one commit, referenced as `US-049.Tn`.

- [ ] T1 — The fix from SP-019's cause, with a test of the reproduced case (2 h)
- [ ] T2 — Test card and guided step (1 h)
