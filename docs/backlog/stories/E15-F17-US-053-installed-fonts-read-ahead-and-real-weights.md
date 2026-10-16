---
id: US-053
epic: E15
feature: F17
title: Installed fonts read ahead, and only the weights the browser draws
status: ready
points: 5
---

# E15 · F17 · US-053 — Installed fonts read ahead, and only the weights the browser draws

As a symbol designer, I want the font and weight lists to offer only what the browser can really draw, so that a text's frame always matches the letters drawn.

From the Product Owner's first pass of VAL-018 (2026-10-16), findings D1, D3 and D5:

- **D5**: the installed families were listed without reading any file; a family whose files Chrome cannot give (ABC Oracle Book) was offered, then refused once chosen.
- **D1**: Inter is not installed on the Product Owner's Mac. The text was measured with Inter 4.1's shipped metrics and drawn with Helvetica, the browser's `sans-serif`: the weight list offered Inter's nine weights while only Helvetica's 300, 400 and 700 were drawn, and the box's padding changed with the weight and the letters.
- **D3**: a right-aligned line ended short of the frame, mostly from D1; a side note asked whether the browser adds a letter spacing after a line's last letter that the measure leaves out.

**Decisions** (provisional, taken while the Product Owner was away, to confirm at VAL-018's second pass; `docs/domain/shapes.md`, Installed fonts and Default font available):

- Once the browser lists the installed fonts, their files are read in the background, family after family, the page answering between files. A family none of whose faces can be read leaves the font list. Only each face's metrics are kept, not its file: a family chosen has its files read again, as before, so that the page draws it with its own outlines and can embed it.
- Inter's shipped metrics are used only for the faces the browser really draws with them. When the page opens, the editor measures a probe line in each of Inter's 18 faces on a canvas, as the drawing names the font (`"Inter", sans-serif`), and keeps the faces the browser draws as wide as Inter 4.1's metrics measure them. A face read from a file, imported or installed, is always kept. When no face of Inter is drawn, Inter stays first in the font list without weights, a text in it is not measured, and the page says Inter is missing and how to get it. A browser that cannot measure on a canvas keeps Inter's 18 faces, as before.
- Letter spacing (D3's side note): the measure adds the spacing between adjacent letters only, never after the line's last letter, as CSS Text 3 §7.2 says ("not applied at the beginning or end of a line", `REF-CSS-TEXT-3`). A test confirms that a right-aligned line ends at the block's right edge. What Chrome draws cannot be checked without a browser here; nothing is changed, and the Product Owner's test checks it.

## Acceptance criteria (F17.AC11)

- Given the installed fonts listed, when their files are read in the background, then each family is read face by face, a line counting the families read (`Reading the installed fonts: 12 of 312 families…`), then `Installed fonts read: 309 families; 3 left out, their files could not be read (ABC Oracle Book, …).`
- Given an installed family none of whose faces the browser can give, when its files are read, then it leaves the font list.
- Given Helvetica read in the background (300, 400 and 700, upright and oblique), when it is chosen, then the weight list offers 300, 400 and 700 only, the text is measured with it at once, and its files are read again: `Helvetica: 6 faces read.`
- Given a face already kept from a file, imported, when its installed family is read in the background, then the face keeps its file.
- Given a browser that draws no face of Inter as Inter 4.1's metrics measure it, when the page opens, then Inter has no weight, a text in Inter is not drawn, and the page says `Inter is not installed on this computer: …`; given it draws Inter 400 and 700 only, then the weight list offers 400 and 700.
- Given Inter's files imported, or Inter installed and read, then its faces read are offered.
- Given "ab" with a letter spacing of 10, aligned right, when it is measured and placed, then the spacing is counted once, between a and b, and the line ends at the block's right edge.

## Product Owner test

| Step | Do                                                                         | You should see                                                                                                |
| ---- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| 1    | Open the page in Chrome, Inter not installed, choose the Text tool         | Inter in the font list without weights, no text drawn, a line saying Inter is not installed and how to get it |
| 2    | Click Installed fonts, allow                                               | the families listed, then a line counting the families read, the page still answering                         |
| 3    | Wait for the end of the reading                                            | `Installed fonts read: … families; … left out, …`; ABC Oracle Book no longer in the font list                 |
| 4    | Choose Helvetica                                                           | `Helvetica: 6 faces read.`; weights 300, 400 and 700 only; the frame fits the letters at each weight          |
| 5    | Import Inter 4.1's files (Inter-Regular.ttf, Inter-Bold.ttf), choose Inter | weights 400 and 700 only; the frame fits the letters                                                          |
| 6    | A text "Running" in Helvetica, letter spacing 20, aligned right            | does the last letter end at the frame's right edge, or 20 short of it? (the measure adds no spacing after it) |

## Tasks

One task = one commit, referenced as `US-053.Tn`.

- [ ] T1 — The installed fonts read in the background, a family that cannot be read left out, metrics kept without files, a family chosen read again; `shapes.md` (3 h)
- [ ] T2 — Inter's faces kept only when the browser draws them, Inter missing said; `shapes.md`, references (2 h)
- [ ] T3 — A test that a right-aligned line with a letter spacing ends at the block's edge (CSS Text 3 §7.2) (0.5 h)
