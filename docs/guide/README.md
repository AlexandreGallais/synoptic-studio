# Guide

What the library does, explained in plain language for the Product Owner and, later, for users. No formula without a sentence that says what it means; no term without a definition or a link to the [glossary](../domain/). Each page ends with **how to try it** in the playground and the **validation** by the Product Owner.

Written at the end of each feature (`VAL`, demo page) and of each epic (`REV`, epic report), ADR-0023. Category: explanation (`REF-DIATAXIS`); the API reference stays austere.

## Writing rules

1. Start with what the user sees, then why, then how — one idea per paragraph.
2. Explain each technical word the first time: "a **fillet** is the small arc that replaces a sharp corner".
3. Numbers from the playground rather than symbols: "a 100 × 25 rectangle with radius 100 gets radius 12.5".
4. A picture (SVG produced by the library, or a sketch) for every geometric idea.
5. Say the limits: what the feature does not do yet, and which feature will.
6. The guided test is a few short steps, each with what to do and what to see; it lives in the playground for the feature being validated only. The Product Owner says what they see; the agent records the result and the remarks in neutral words (`docs/conventions/writing.md`).
7. At the epic review, every step of the epic comes back at once: `?guide=epic` in the playground's address.

## Pages

| Page                                                                                 | Content                                                                                |
| ------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| [F01 — A rectangle with rounded corners](./f01-rounded-rectangle.md)                 | rounding corners, the radius reduced to fit, the guided test                           |
| [F02 — A regular polygon with rounded corners](./f02-regular-polygon.md)             | the polygon fitted in its box, the inscribed circle, sizes, the guided test            |
| [F07 — A polygon anchored in its box](./f07-polygon-anchor.md)                       | the nine anchor buttons, the intent kept, rounding inwards, the guided test            |
| [F03 — A rounding for each corner](./f03-radius-per-vertex.md)                       | points mode, point 1 and 2, choosing corners, Mixed, the guided test                   |
| [F09 — The rectangle, a free shape you can stretch](./f09-stretchable-free-shape.md) | points in a box of their own, stretching without loss, the guided test                 |
| [F08 — Moving, adding and deleting points](./f08-node-editing.md)                    | converting a polygon, typing x and y, a point in the middle, deleting, the guided test |
| [F04 — Turning a shape by any angle](./f04-free-rotation.md)                         | the angle, the two boxes, the position, editing a turned shape, the guided test        |
| [F06 — A polygon stretched to its box](./f06-stretched-polygon.md)                   | the **Stretched to its box** check box, rule C in five boxes, the guided test          |
| [F11 — A code base ready for curves](./f11-code-base-ready-for-curves.md)            | the figures before and after, the folders by domain, how to check them                 |
| [F12 — Points on a curve, with handles](./f12-curves-with-handles.md)                | the options of handles, a corner beside a curve, the guided test, what to decide       |
| [F10 — Round shape](./f10-round-shape.md)                                            | the Round tool, its two conversions, the guided test, what the agent decided           |
| [E01 — Drawing symbol shapes from numbers](./e01-draw-shapes.md)                     | the eight features of E01, the words, a user test crossing them, the decisions         |

Templates: `_feature-demo.md`, `_epic-report.md` (not published).
