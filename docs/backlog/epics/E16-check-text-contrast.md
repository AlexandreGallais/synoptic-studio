---
id: E16
title: Check the contrast of texts on their background
status: draft
---

# E16 — Check the contrast of texts on their background

An idea to refine, recorded on 2026-10-15 from the Product Owner's description. Not part of E15 and not in the run order: the Product Owner places it later. No research done yet. Its edition, free V1 or pro, is to be decided with the Product Owner (ADR-0030).

| Field                       | Content                                                                                                                                                                  |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| For                         | users who place texts in symbols and views                                                                                                                               |
| who                         | need to know whether a text stays readable on what is behind it                                                                                                          |
| the solution                | a contrast checker                                                                                                                                                       |
| is a                        | a tool that measures a text's color against its background, in symbols, views and their animations                                                                       |
| that                        | gives two results side by side, the WCAG 2 contrast ratio and the APCA lightness contrast, and says in plain words whether the text is readable for its size and weight  |
| unlike                      | judging contrast by eye, or reading a bare number                                                                                                                        |
| our solution                | each text gets a verdict a non-expert understands, such as readable or too pale for a bold 16 px text                                                                    |
| Business outcomes           | to refine                                                                                                                                                                |
| Leading indicators          | the spike has read the sources below                                                                                                                                     |
| Non-functional requirements | to refine                                                                                                                                                                |
| In scope                    | WCAG 2 contrast ratio with levels AA and AAA and the large-text thresholds by size and weight; APCA, the WCAG 3 draft's lightness contrast, read by font size and weight |
| Out of scope                | to refine                                                                                                                                                                |
| Closure criteria            | to refine                                                                                                                                                                |

## Notes for later

- **Two methods side by side.** The result carries two pieces of information, one per method: the WCAG 2 level and the APCA reading.
- **Plain words.** The verdict says whether the text is readable for its size and weight, not only a number.
- **Later option: a minimum contrast at all times.** A text may require a minimum contrast whatever its color or its background becomes through dynamic color changes (animations).
- **Sources to read at its spike** (none read yet, none cited): WCAG 2.2 success criteria 1.4.3 (Contrast, Minimum) and 1.4.6 (Contrast, Enhanced); APCA.

## Candidate features

| Feature                         | Status    |
| ------------------------------- | --------- |
| Contrast of a text, two methods | to refine |
| Readability verdict in words    | to refine |
| Minimum contrast at all times   | to refine |
