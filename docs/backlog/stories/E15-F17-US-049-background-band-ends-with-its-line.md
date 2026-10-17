---
id: US-049
epic: E15
feature: F17
title: A wrapped text's background band ends with its line
status: done
points: 2
---

# E15 · F17 · US-049 — A wrapped text's background band ends with its line

As a symbol designer, I want each line's background to end where the line ends so that a highlighted text in a box looks right.

From the Product Owner's validation of E15 (2026-10-15): the band behind a wrapped line may run past the line's end, to the right, on a new line. Reproduced by SP-019 ([research note 0021](../../research/notes/0021-f17-latin-script-and-defects.md)): a line ended by a wrap loses its trailing spaces, but the last line of a paragraph keeps them (`wrappedParagraph`), so its width and its band count them, as `DERIV-text-box` step 2 chose. In Arial 16 in a box 50 wide, "Hello World " gives "World " 46.227 wide, its band 4.445 past "World" (41.781), one space; with two spaces, 50.672, past the box. A space typed before the next word shows it on the new line.

Open for the Product Owner at the start of this story (`DERIV-text-box`, step 2): whether the last line's trailing spaces still place a right-aligned or centred line. Hanging, as CSS `white-space: normal`: "World " right-aligned in the box 50 sits at 8.219. Kept for placement: it sits at 3.773, a space's width from the edge. The band ends with "World" either way.

Decided on 2026-10-16, provisional until VAL-018: the trailing spaces of a paragraph's last line are ignored for placing the line and for its band, as every other line's — "World " right-aligned in the box 50 sits at 8.219, its band ending with "World" (`DERIV-text-box`, step 2).

## Acceptance criteria

- Given "Hello World " (a space at the end) in Arial 16 in a box 50 wide with a background, when drawn, then the band of "World" is 41.781 wide, not 46.227; with two spaces, never past the box; each band runs from the line's start to its last character other than a space, its ellipsis included.
- Given "Hello World" in Arial 16 in a box 50 wide with a background, when drawn, then the band of "Hello" is as long as "Hello" and that of "World" as long as "World".
- Given a text fitting its content with a background, when drawn, then its bands are as before (F14.AC4).

## Product Owner test

| Step | Do | You should see |
| --- | --- | --- |
| 1 | "Hello World " (a space at the end), background on | each band ending with its word |
| 2 | "Hello World", Arial 16, in a box 50 × 40, background on | two bands, each as long as its word |

## Tasks

One task = one commit, referenced as `US-049.Tn`.

- [x] T1 — The band of a last line without its trailing spaces, the Product Owner's answer on placement written in `DERIV-text-box` step 2, with a test of the reproduced case (2 h)
- [x] T2 — Test card and guided step (1 h)
