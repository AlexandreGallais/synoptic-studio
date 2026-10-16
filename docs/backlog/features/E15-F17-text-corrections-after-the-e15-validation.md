---
id: F17
epic: E15
title: Text corrections after the E15 validation
status: ready
---

# E15 · F17 — Text corrections after the E15 validation

**Benefit hypothesis**: fewer options make the text clearer to use, and the defects found at the validation of E15 are gone, so that a designer can place text in a symbol without surprise.

Written on 2026-10-15 from the Product Owner's validation of E15 (VAL-014 to VAL-017): three options removed, embedded fonts moved to export, five defects to fix, and a new validation format.

## Acceptance criteria

1. A word too long for its box always stays whole, overflowing it or cut with "…"; the lines of a box always wrap automatically, the balanced wrap removed (F14's criteria 1 and 3 reworded).
2. No text is written with an HTML layer: no `switch` nor `foreignObject` (ADR-0060); F15's criterion 3 withdrawn.
3. Embedded fonts are written only when a file is exported or saved, never in the drawing on screen.
4. An installed font held in a collection, or whose access failed with a `TypeError`, is read, or refused with its reason said; a weight such as 450 is drawn with the face it is measured with.
5. A value changed fast, an arrow key held, is redrawn at most once per animation frame, only what the change touches computed again.
6. A wrapped text's background bands end with their lines.
7. A turned text shows its turned frame in blue and its straight box dashed, as a turned shape does.
8. A feature is validated on one checklist page with check boxes; the `run` and `review` skills prepare it.
9. Every shape and text draws as before, apart from the options removed.
10. A large font collection is read without running out of memory: a face of a collection is given as views of its tables, only the face kept is assembled, and every error of a read is said in one line (VAL-018, 2026-10-16: Apple Color Emoji, 192 MB, crashed the page).
11. The font and weight lists offer only what the browser can draw: the installed fonts are read in the background, a family that cannot be read left out, and Inter's shipped metrics used only for the faces the browser draws with them, Inter missing said (VAL-018, 2026-10-16: Inter measured, Helvetica drawn).

## Stories

In delivery order.

| ID                                                                                         | Type       | Title                                                                                | Status |
| ------------------------------------------------------------------------------------------ | ---------- | ------------------------------------------------------------------------------------ | ------ |
| [SP-019](../stories/E15-F17-SP-019-research-f17-latin-script-and-defects.md)               | Spike      | Research for F17: the Latin script in industrial HMIs, and the causes of the defects | done   |
| [US-042](../stories/E15-F17-US-042-a-long-word-always-whole.md)                            | User story | A word too long always stays whole                                                   | done   |
| [US-043](../stories/E15-F17-US-043-no-balanced-wrap.md)                                    | User story | The balanced wrap removed, lines always wrapped automatically                        | done   |
| [US-044](../stories/E15-F17-US-044-no-html-layer.md)                                       | User story | The HTML layer removed                                                               | done   |
| [US-045](../stories/E15-F17-US-045-fonts-embedded-at-export.md)                            | User story | Fonts embedded only when a file is exported or saved                                 | done   |
| [US-046](../stories/E15-F17-US-046-installed-fonts-from-collections.md)                    | User story | Installed fonts read from collections, and faces that failed                         | done   |
| [CHK-018](../stories/E15-F17-CHK-018-checkpoint-f17.md)                                    | Checkpoint | Checkpoint in the middle of F17                                                      | done   |
| [US-047](../stories/E15-F17-US-047-weight-450-drawn-as-measured.md)                        | User story | A weight such as 450 drawn with the face it is measured with                         | done   |
| [US-048](../stories/E15-F17-US-048-fluid-changes-held-keys.md)                             | User story | Values changed fast redrawn without lag                                              | done   |
| [US-049](../stories/E15-F17-US-049-background-band-ends-with-its-line.md)                  | User story | A wrapped text's background band ends with its line                                  | done   |
| [US-050](../stories/E15-F17-US-050-turned-text-frames-as-shapes.md)                        | User story | A turned text's frames drawn as a turned shape's                                     | done   |
| [US-051](../stories/E15-F17-US-051-validation-checklist-page.md)                           | User story | Validation on one checklist page                                                     | done   |
| [EN-044](../stories/E15-F17-EN-044-val-steps-as-a-checklist.md)                            | Enabler    | The run and review skills validate with a checklist page                             | done   |
| [US-052](../stories/E15-F17-US-052-large-collection-read-without-running-out-of-memory.md) | User story | Read a large font collection without running out of memory                           | done   |
| [US-053](../stories/E15-F17-US-053-installed-fonts-read-ahead-and-real-weights.md)         | User story | Installed fonts read ahead, and only the weights the browser draws                   | ready  |
| [AUD-019](../stories/E15-F17-AUD-019-audit-f17.md)                                         | Audit      | Audit F17                                                                            | done   |
| [VAL-018](../stories/E15-F17-VAL-018-validate-f17.md)                                      | Validation | Validate F17                                                                         | ready  |

## Feature plan

| Acceptance criterion | Realized by            | Verified by         |
| -------------------- | ---------------------- | ------------------- |
| F17.AC1              | US-042, US-043         | AUD-019, VAL-018    |
| F17.AC2              | US-044                 | AUD-019, VAL-018    |
| F17.AC3              | US-045                 | AUD-019, VAL-018    |
| F17.AC4              | SP-019, US-046, US-047 | AUD-019, VAL-018    |
| F17.AC5              | SP-019, US-048         | AUD-019, VAL-018    |
| F17.AC6              | SP-019, US-049         | AUD-019, VAL-018    |
| F17.AC7              | US-050                 | AUD-019, VAL-018    |
| F17.AC8              | US-051, EN-044         | VAL-018             |
| F17.AC9              | every story            | `npm run check:all` |
| F17.AC10             | US-052                 | VAL-018             |
| F17.AC11             | US-053                 | VAL-018             |

## Left for later

- A search field over the installed fonts, a reset button on the stroke colors instead of the box "follows the text's color", and fewer options, the stroke colors perhaps among them: a later epic on the editor's ease of use (E02, ideas).
