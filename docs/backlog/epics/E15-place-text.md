---
id: E15
title: Place text in symbols
status: in-progress
---

# E15 — Place text in symbols

Decided by the Product Owner on 2026-10-12: text is too large a subject for one feature of E01, so F05 left E01 for this epic, after E14.

| Field                       | Content                                                                                    |
| --------------------------- | ------------------------------------------------------------------------------------------ |
| For                         | symbol designers                                                                           |
| who                         | need labels and values in their symbols                                                    |
| the solution                | a text tool                                                                                |
| is a                        | text with content, font, integer size, anchor and color, measured so that it can be placed |
| that                        | text that stays where it is meant to be whatever the font                                  |
| unlike                      | a text box whose real size is only known once drawn                                        |
| our solution                | the way text is drawn and measured is chosen from sources, before any code                 |
| Business outcomes           | a symbol shows a text with the font, size and anchor chosen, at the place expected         |
| Leading indicators          | the text approach decided (Q31); reference cases of text accepted                          |
| Non-functional requirements | no runtime dependency; integer inputs (ADR-0003)                                           |
| In scope                    | text content, font, size, anchor, color; measuring the size of a text                      |
| Out of scope                | text converted to paths                                                                    |
| Closure criteria            | the text cases of `shapes.md` render as specified in the playground                        |

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

| Feature                                   | Status      |
| ----------------------------------------- | ----------- |
| [F05](../features/E15-F05-text.md) — Text | in-progress |
