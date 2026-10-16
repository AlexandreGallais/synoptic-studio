---
id: E15
title: Place text in symbols
status: in-progress
---

# E15 — Place text in symbols

Decided by the Product Owner on 2026-10-12: text is too large a subject for one feature of E01, so F05 left E01 for this epic, after E14.

| Field                       | Content                                                                                                                                                                                                    |
| --------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| For                         | symbol designers                                                                                                                                                                                           |
| who                         | need labels and values in their symbols                                                                                                                                                                    |
| the solution                | a text tool                                                                                                                                                                                                |
| is a                        | text with content, font, integer size, anchor and color, measured so that it can be placed                                                                                                                 |
| that                        | text that stays where it is meant to be whatever the font                                                                                                                                                  |
| unlike                      | a text box whose real size is only known once drawn                                                                                                                                                        |
| our solution                | the way text is drawn and measured is chosen from sources, before any code                                                                                                                                 |
| Business outcomes           | a symbol shows a text with the font, size and anchor chosen, at the place expected                                                                                                                         |
| Leading indicators          | the text approach decided (Q31); reference cases of text accepted                                                                                                                                          |
| Non-functional requirements | no runtime dependency; integer inputs (ADR-0003)                                                                                                                                                           |
| In scope                    | text and its characters, a box of text, colors and their pickers, fonts from files, embedded fonts at export; measuring text from font metrics; the Latin script, with Greek, Cyrillic and Hebrew as built |
| Out of scope                | text converted to paths; an HTML layer for browsers, Arabic and Indic scripts (Product Owner, 2026-10-15, ADR-0060)                                                                                        |
| Closure criteria            | the text cases of `shapes.md` render as specified in the playground                                                                                                                                        |

## To study in the first spike

- Native SVG `<text>` (ADR-0002) and its own features, or HTML text inside a `foreignObject` (related to Q24), and the drawbacks of each (rendering in other tools, export, printing).
- Fonts: which fonts are allowed, how they are named and found, what happens when one is missing.
- Size: how the size of a text is measured without the DOM in the portable core (ADR-0025), for the layout box (E13) and alignment.

## Before E15's release

- **A release without an extra commit on `main`** (Product Owner, 2026-10-14, after the 0.2.0 release). Today release-please runs on `main`: once the release pull request from `develop` is merged, it opens a second pull request that bumps the version and the changelog. That makes one more commit on `main`, which `develop` then has to merge back. The plan, to check against release-please's documentation:
  - run its `release-pr` step on `develop` (target branch `develop`), so that the version and the changelog are bumped there;
  - run only its `github-release` step on `main`, which tags and publishes the release from the merged commit.

  `main` then never holds a commit that `develop` lacks, and `develop` follows `main` by a fast-forward. `develop` stays: it is the integration branch.

## Features

| Feature                                                                                                             | Status |
| ------------------------------------------------------------------------------------------------------------------- | ------ |
| [F05](../features/E15-F05-text.md) — Text and its characters                                                        | done   |
| [F14](../features/E15-F14-a-box-of-text.md) — A box of text                                                         | done   |
| [F16](../features/E15-F16-colors.md) — Colors and their pickers                                                     | done   |
| [F15](../features/E15-F15-fonts-from-files-and-embedded.md) — Fonts from files, embedded fonts and the HTML layer   | done   |
| [F17](../features/E15-F17-text-corrections-after-the-e15-validation.md) — Text corrections after the E15 validation | ready  |

Split by the agent on 2026-10-14, as the Product Owner asked: the characters first, then the box, the colors, and the fonts with the HTML layer, which needs a new ADR. The Product Owner validated the four on 2026-10-15; F17 holds the corrections of that validation: two options and the HTML layer removed (ADR-0060), fonts embedded at export, five defects fixed, and a validation on one checklist page.

## Closing

After its last feature is validated (ADR-0023, ADR-0049).

| Story                                                                                   | Status |
| --------------------------------------------------------------------------------------- | ------ |
| [NAM-001](../stories/E15-NAM-001-naming-pass-e15.md) — Naming pass over the code of E15 | ready  |
| [REV-003](../stories/E15-REV-003-review-e15.md) — Review E15 with the Product Owner     | draft  |
| [RET-003](../stories/E15-RET-003-retrospective-e15.md) — Retrospective of E15           | draft  |
