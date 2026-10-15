---
id: US-041
epic: E15
feature: F16
title: Choose the underline's and the strike-through's colors
status: ready
points: 3
---

# E15 · F16 · US-041 — Choose the underline's and the strike-through's colors

As a designer, I want to choose the color and the opacity of a text's underline and of its strike-through, with the same color field and picker as the text's, so that a crossed-out value stands out in red over a black text.

Product Owner decision of 2026-10-15, from a try of the playground before VAL-014. Rules: `shapes.md` (Text, Lines drawn), `interaction.md` §5 (Colors), ADR-0056.

Built on the latest branch of the chain, `feature/f15-fonts-html`, after US-036.

## Decision to confirm at VAL-016

The agent's reading: until the designer chooses one, each decoration's color follows the text's color and opacity, so that every text of F05 draws as before; once chosen, it is kept when the text's color changes. The alternative: each decoration starts with a copy of the text's color, kept apart from then on.

## Acceptance criteria

- Given a text in `#000000` at 100 %, underlined and struck through, its underline set to `#ff0000` at 50 %, when drawn, then the underline's path has `fill="#ff0000"` and `fill-opacity="0.5"`, and the strike-through's path the text's `#000000` and 1.
- Given a text whose decorations' colors were never chosen, when its color changes to `#0000ff`, then both strokes are drawn in `#0000ff`, as in F05.
- Given an underline color typed as `F00` in its field, then it is kept as `#ff0000`, as every color is (EN-037); given `red`, then it is refused, the field showing its previous color.
- Given the underline's swatch, when clicked, then the same picker opens, with its hue, opacity and plane.
- Given a text read with an underline color that is not `#` and six lower-case digits, or an opacity outside 0 to 100, then it is refused (`isValidText`).
- Given every text of F05, F14 and F16, then each draws as before.

## Product Owner test

Open `http://localhost:5173/`, click **Text**, click on the drawing, type `P-101`, tick **Underline** and **Strike-through**.

| Step | Do                                                       | You should see                                |
| ---- | -------------------------------------------------------- | --------------------------------------------- |
| 1    | Look at the strokes                                      | both in the text's color, as before           |
| 2    | Set the strike-through's color to `#ff0000`, opacity 100 | the strike-through red, the underline black   |
| 3    | Open the underline's swatch, pick a blue at 50 %         | the underline see-through blue                |
| 4    | Change the text's color to green                         | the letters green; the strokes keep red, blue |

## Tasks

One task = one commit, referenced as `US-041.Tn`.

- [ ] T1 — The underline's and the strike-through's colors and opacities in the model, following the text's until chosen, checked as every color (EN-037); tests first (2 h)
- [ ] T2 — Each stroke drawn with its own `fill` and `fill-opacity`; the rendering tests first (2 h)
- [ ] T3 — The playground's two color fields with their swatch and picker; the guided steps and the test card; the playground's tests first (3 h)
