---
id: US-034
epic: E15
feature: F14
title: A background color behind a text, as a highlighter
status: ready
points: 2
---

# E15 · F14 · US-034 — A background color behind a text, as a highlighter

As a designer, I want a background color behind a text's lines, as a highlighter, so that a label stays readable on a busy synoptic view.

## Acceptance criteria

- Given "Hello" over "World" in Arial 16, kerning left out, at (0, 0), aligned left, with a background, when drawn, then one `<path>` holds two rectangles, clockwise from their top-left corners (Q11): (0, 0) 36.46094 × 17.875 and (0, 18.39844) 41.78125 × 17.875, from the ascender to the descender of each line (`DERIV-text-box`, step 10; `REF-CSS-INLINE-3`, §6.4).
- Given a background, when drawn, then its `<path>` comes before the `<text>` in the text's group, with the background's color as `fill` and its opacity from 0 to 1 as `fill-opacity`, and it turns with the group (ADR-0055).
- Given a line cut with "…", when drawn, then its rectangle runs to the end of the ellipsis; given an empty line, then it has no rectangle.
- Given no background, when drawn, then no `<path>` is added and the text draws as before (F14.AC6).

## Product Owner test

Open `http://localhost:5173/`, click **Text** and type **Size** 16 (Inter, weight 400, Fit content, as the page opens).

| Step | Do                                                                                                                      | You should see                                                                                                                            |
| ---- | ----------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| 1    | Type **Content** "Pump P-101", a line break, "Running"; tick **Background**                                             | a yellow band behind each line, as long as the line: 87.95 behind "Pump P-101", 61.80 behind "Running", each 19.36 tall, the two touching |
| 2    | Type **Background color** `#22c55e`, **Background opacity** 40                                                          | the bands turn light green, the letters stay on top; the SVG shows one `<path … fill="#22c55e" fill-opacity="0.4">` before the `<text>`   |
| 3    | Type **Content** "Pump P-101", two line breaks, "Running"                                                               | two bands only: the empty line in between has none                                                                                        |
| 4    | Type **Content** "Pump P-101", one line break, "Running"; choose **Box**, **Width** 50, **Height** 100, **Max lines** 2 | "Pump" over "P-10…": bands 43.49 and 46.88 long, the second up to the end of its "…"                                                      |
| 5    | Type **Angle** 30                                                                                                       | the bands turn with the text                                                                                                              |
| 6    | Type **Background opacity** 101                                                                                         | nothing is drawn, the message names the background opacity                                                                                |
| 7    | Type **Background opacity** 40 again, untick **Background**                                                             | the bands go; the text is as before                                                                                                       |

## Tasks

One task = one commit, referenced as `US-034.Tn`.

- [x] T1 — The background's rectangles per line in the text's drawing, one `<path>` under the `<text>`, `@see DERIV-text-box`; tests first (2 h)
- [x] T2 — The Background, Background color and Background opacity fields in the playground; the test card checked (1 h)
