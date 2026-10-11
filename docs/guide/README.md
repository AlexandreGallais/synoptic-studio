# Guide

What the library does, explained in plain language for the Product Owner and, later, for users. No formula without a sentence that says what it means; no term without a definition or a link to the [glossary](../domain/). Each page ends with **how to try it** in the [playground](https://alexandregallais.github.io/synoptic-studio/playground/).

Written at the end of each feature (`VAL`, demo page) and of each epic (`REV`, epic report), ADR-0023. Category: explanation (`REF-DIATAXIS`); the API reference stays austere.

## Writing rules

1. Start with what the user sees, then why, then how — one idea per paragraph.
2. Explain each technical word the first time: "a **fillet** is the small arc that replaces a sharp corner".
3. Numbers from the playground rather than symbols: "a 100 × 25 rectangle with radius 100 gets radius 12.5".
4. A picture (SVG produced by the library, or a sketch) for every geometric idea.
5. Say the limits: what the feature does not do yet, and which feature will.
6. The guided test is a list of steps with the expected result of each; the Product Owner says what they see, and the agent records it, quoting them.

## Pages

| Page                                                                     | Content                                                                     |
| ------------------------------------------------------------------------ | --------------------------------------------------------------------------- |
| [F01 — A rectangle with rounded corners](./f01-rounded-rectangle.md)     | rounding corners, the radius reduced to fit, the guided test                |
| [F02 — A regular polygon with rounded corners](./f02-regular-polygon.md) | the polygon fitted in its box, the inscribed circle, sizes, the guided test |
| [F07 — A polygon anchored in its box](./f07-polygon-anchor.md)           | the nine anchor buttons, the intent kept, rounding inwards, the guided test |

Templates: `_feature-demo.md`, `_epic-report.md` (not published).
