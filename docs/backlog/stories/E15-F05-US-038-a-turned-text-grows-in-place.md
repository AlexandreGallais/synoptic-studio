---
id: US-038
epic: E15
feature: F05
title: A turned text grows along its direction and stays in place
status: ready
points: 2
---

# E15 · F05 · US-038 — A turned text grows along its direction and stays in place

As a designer, I want a turned text to stay where I put it when I type in it or change its spacing, its size or its font, its width growing along its direction and its height with its lines, so that a label turned beside a pipe keeps its place.

Product Owner decision of 2026-10-15 (ADR-0057), built on EN-042. The text still turns about its middle. Built on the latest branch of the chain, `feature/f15-fonts-html`, after US-036 and EN-042.

## Acceptance criteria

- Given a text "Pump P-101" in Inter 16 at (47, 61), fitting its content, turned by 23, when the designer raises its letter spacing from 0 to 10, then its turned top-left corner keeps its coordinates on the drawing, its position fields follow the position computed again, and the text grows along its direction.
- Given the same text, when the designer types a line break and a second line, then the corner stays and the text grows on the side of its lines.
- Given the same text, when the designer changes its size from 16 to 32, or its font, then the corner stays.
- Given the same text in a box 120 × 40, when the designer raises its letter spacing, then nothing moves; when they set the box's width to 160, then the corner stays.
- Given the turned text selected, then its dashed frame turns with it, about the same middle.
- Given every text and shape of E01, E14, F05, F14 and F16 at 0°, then each draws as before.

## Product Owner test

Open `http://localhost:5173/`, click **Text**, click on the drawing, type `Pump P-101`, choose the size 16 and the weight Regular, set X 47, Y 61 and the angle 23.

| Step | Do                                                    | You should see                                                                                        |
| ---- | ----------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| 1    | Raise the letter spacing from 0 to 10, step by step   | the P's turned top-left corner stays put, the text grows along its line; at 10, X 43.42272, Y 78.5829 |
| 2    | Add a second line                                     | the corner stays; the text grows on the side of its lines; X 39.64056, Y 77.81341                     |
| 3    | Set the size to 32, then the weight Bold              | the corner stays                                                                                      |
| 4    | Set the angle to 40                                   | the text turns about its middle; X and Y do not change                                                |
| 5    | Set the box 120 × 40, choose **Box**, then X 47, Y 61 | choosing Box keeps the corner; the text in its box at (47, 61)                                        |
| 6    | Set the angle to 23, raise the letter spacing         | nothing moves: the box keeps its size                                                                 |
| 7    | Set the box's width to 160                            | the corner stays; X 45.4101, Y 68.81462                                                               |
| 8    | Click **Rectangle**, then **Text**                    | X and Y as they were; a spacing changed then keeps the corner                                         |
| 9    | Set the angle to 0, raise the letter spacing          | X and Y do not change: the text grows to the right and down                                           |

In the guided test (`?guide=epic`), the steps **F05 · A turned text grows in place (US-038)** and **F14 · A turned box resized in place (US-038)** walk through steps 1 and 6 to 7.

## Tasks

One task = one commit, referenced as `US-038.Tn`.

- [ ] T1 — The playground computes the new position through EN-042 whenever a turned text's block or box changes size (content, spacing, size, font, line height, box), and shows it in x and y; the playground's tests first (2 h)
- [ ] T2 — The guided steps of F05 and F14 (turned text, the dashed frame) and VAL-014's examples brought up to date; the test card (1 h)
