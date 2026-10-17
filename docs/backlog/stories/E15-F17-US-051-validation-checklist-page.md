---
id: US-051
epic: E15
feature: F17
title: Validation on one checklist page
status: done
points: 2
---

# E15 · F17 · US-051 — Validation on one checklist page

As the Product Owner, I want one page listing every feature delivered, with check boxes, so that I test freely instead of walking long guided steps.

From the Product Owner's validation of E15 (2026-10-15): the long step-by-step guided test is replaced by one checklist page, without Previous nor Next.

## Acceptance criteria

- Given the playground, when the checklist of an epic is opened (its address said in the demo page), then one page lists every feature delivered in it, one line each: what to try and what to see, a check box before it, no Previous nor Next.
- Given E15, when its checklist is opened, then it lists every option kept after F17, about 40 lines, and none of the options removed (the balanced wrap, the word broken at a character, the HTML layer).
- Given a box ticked, when the page is reloaded, then the box stays ticked in that browser.

## Product Owner test

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Open E15's checklist | one page of lines with check boxes, no Next button |
| 2 | Tick two lines, reload | both still ticked |

## Tasks

One task = one commit, referenced as `US-051.Tn`.

- [x] T1 — The checklist page in the playground, its lines from a list, tested (3 h)
- [x] T2 — E15's checklist written from the delivered features (2 h)
