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

**Decision** (Product Owner, 2026-10-16, questionnaire after the first fix; `docs/domain/shapes.md`, Text, Box): in Box mode the designer sets the box's size, and a switch to Fit content and back to Box keeps it — a box typed 100 × 200, Fit content to try something, then Box, is 100 × 200 again. A **Reset** button shown in Box mode puts the box back to the fitted size, the one Fit content measures rounded up (US-055), and the box then counts as not typed; it changes nothing else in that rule. The first switch to Box takes the fitted size; the maximum number of lines alone does not make the box typed and stays kept. **What forgets it** (agent's choice): a new text — the page reopened or a guided step, which types its own text; neither the content typed nor the choice of another tool forgets it.

**Provisional** (the agent's reading, 2026-10-16, to confirm at VAL-018): a width or height field the designer emptied does not count as typed — back in Box, that side takes the measured size, not 0, the other side kept as typed.

## Acceptance criteria (F17.AC15)

- Given "Pump P-101" in Inter 16 put in a box, 88 × 20, when its height is typed 15, Fit content chosen, then Box, then the box is 88 × 15.
- Given that box 88 × 20, when its width is typed 50, Fit content chosen, a second line "Tank" typed, then Box chosen, then the box is 50 × 20, not 88 × 39.
- Given that box, its height typed 40 and its maximum number of lines 1, when Fit content then Box is chosen, then the box is 88 × 40 and the maximum is still 1.
- Given a box never typed, when Fit content, a second line, then Box is chosen, then the box is measured again, 88 × 39 (US-055).
- Given that box typed 100 × 200, when Reset is pressed, then the box is 88 × 20; when Fit content, a second line "Tank" and Box are then chosen, the box is measured again, 88 × 39.
- Given that box, its height typed 15, when another tool is chosen, then the Text tool, Fit content and Box, then the box is 88 × 15.
- Given that box, only its maximum number of lines typed 1, when Fit content, a second line "Tank" and Box are chosen, then the box is measured again, 88 × 39.
- Given that box typed 50 × 15, when its width is emptied, Fit content then Box chosen, then the box is 88 × 15 (provisional).
- Given the guided step "Fit content or Box", its box typed 50 wide, when the step is typed again (Previous, Next) and Box chosen, then the box is measured again, 88 × 20.

## Product Owner test

| Step | Do                                                                 | You should see                                               |
| ---- | ------------------------------------------------------------------ | ------------------------------------------------------------ |
| 1    | Text tool, Fit content, type "Pump P-101", size 16, then Box       | Width 88, Height 20 (US-055)                                 |
| 2    | Type 15 in Height                                                  | the box 88 × 15                                              |
| 3    | Choose Fit content, add a second line "Tank", then choose Box      | Width 88, Height 15 again: the box typed, not measured again |
| 4    | Press Reset                                                        | Width 88, Height 39: the size the two lines fit              |
| 5    | Reopen the page, repeat step 1, then Fit content and Box once more | Width 88, Height 20 each time: a box never typed is measured |

## Tasks

One task = one commit, referenced as `US-057.Tn`.

- [x] T1 — The box typed kept when the text comes back to Box, forgotten by a new text; `shapes.md`, the checklist line (1 h)
- [x] T2 — The Product Owner's decision: a Reset button in Box mode back to the fitted size, the box then not typed; another tool and the maximum number of lines alone keep the rule; an emptied field not typed (provisional); `shapes.md`, the story, a checklist line (1 h)
