---
id: US-057
epic: E15
feature: F17
title: The box typed comes back when the text returns to Box
status: in-progress
points: 1
---

# E15 · F17 · US-057 — The box typed comes back when the text returns to Box

As a symbol designer, I want the box size I typed to come back when I switch a text to Fit content and back to Box, so that trying Fit content never loses the box I set.

From the Product Owner's second pass of VAL-018 (2026-10-16), item F3: Fit content, then Box, then the height reduced, then Fit content, then Box again lost the width and height typed: the box was measured again. The cause: `writeFittedBox` (US-055) writes the text's block rounded up in the box's fields at each switch to Box, without knowing whether the designer typed them since.

**Decision** (provisional, taken by the agent on 2026-10-16, to confirm when the Product Owner checks it; `docs/domain/shapes.md`, Text, Box): the first switch to Box takes the fitted size rounded up (US-055); once the designer has typed the box's width or height, a later switch back to Box restores the box as typed instead of measuring again, whatever changed meanwhile, the content included. The maximum number of lines stays kept, as before. **What resets it**: a new text. The playground edits one text, so a new text is the page reopened or a guided step, which types its own text; neither the content typed nor the choice of another tool resets it, so that a box set once is never lost by editing the text.

## Acceptance criteria (F17.AC15)

- Given "Pump P-101" in Inter 16 put in a box, 88 × 20, when its height is typed 15, Fit content chosen, then Box, then the box is 88 × 15.
- Given that box 88 × 20, when its width is typed 50, Fit content chosen, a second line "Tank" typed, then Box chosen, then the box is 50 × 20, not 88 × 39.
- Given that box, its height typed 40 and its maximum number of lines 1, when Fit content then Box is chosen, then the box is 88 × 40 and the maximum is still 1.
- Given a box never typed, when Fit content, a second line, then Box is chosen, then the box is measured again, 88 × 39 (US-055).
- Given the guided step "Fit content or Box", its box typed 50 wide, when the step is typed again (Previous, Next) and Box chosen, then the box is measured again, 88 × 20.

## Product Owner test

| Step | Do                                                                 | You should see                                               |
| ---- | ------------------------------------------------------------------ | ------------------------------------------------------------ |
| 1    | Text tool, Fit content, type "Pump P-101", size 16, then Box       | Width 88, Height 20 (US-055)                                 |
| 2    | Type 15 in Height                                                  | the box 88 × 15                                              |
| 3    | Choose Fit content, add a second line "Tank", then choose Box      | Width 88, Height 15 again: the box typed, not measured again |
| 4    | Reopen the page, repeat step 1, then Fit content and Box once more | Width 88, Height 20 each time: a box never typed is measured |

## Tasks

One task = one commit, referenced as `US-057.Tn`.

- [x] T1 — The box typed kept when the text comes back to Box, forgotten by a new text; `shapes.md`, the checklist line (1 h)
