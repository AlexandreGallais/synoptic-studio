---
id: EN-041
epic: E15
feature: F15
title: The HTML layer of a text in a switch, over its positioned lines
status: ready
points: 5
---

# E15 · F15 · EN-041 — The HTML layer of a text in a switch, over its positioned lines

The library writes a text, when the document's HTML layer is on, as a `switch`: first a `foreignObject` for browsers, holding the text's content in XHTML with the CSS of its options; then its group of positioned lines, which other readers draw (ADR-0053, `DERIV-html-layer`, research note 0020).

## Acceptance criteria

- Given a text "Hello" over "World" in Arial 16, at (0, 0), the HTML layer on, when written, then an outer group carries the text's turn and holds one `switch` whose first child is a `foreignObject` with `requiredExtensions="http://www.w3.org/1999/xhtml"` and whose second is the text's group as F05 writes it, with the same content and no `transform`; turned by 90°, the outer group carries `rotate(90 …)` and neither child does (`REF-SVG2-STRUCT`, §5.6.3; `REF-SVG2-EMBEDDED`, §12.4).
- Given that text, its line height automatic, when written, then the `foreignObject` is 41.78125 × 36.796875 (its block), its top 0.26171875 above the block's (`DERIV-html-layer`, step 4), and its `div` has `line-height: 18.3984375px`; given a line height of 24, then its top is the block's; trimmed, its top is 3.28515625 above the block's and its height stays 36.796875; in a box 50 × 40 at (0, 0), it is the box raised by 0.26171875, its bottom at 39.73828125.
- Given a text's options, when written, then the `div`'s style carries its font list as the `<text>`'s (ADR-0054), `font-size`, `font-weight`, `font-style`, its color with its opacity, `letter-spacing` and `word-spacing` when not 0, `direction: rtl` for right to left, `text-align` from its alignment, underline and strike-through as `text-decoration-line` with `text-decoration-thickness: from-font` (`REF-MDN-TEXT-DECORATION-THICKNESS`), and the content written in its case, its characters escaped.
- Given a text that fits its content, when written, then its `div` has `white-space: pre` (`REF-MDN-WHITE-SPACE`); given a text in a box 50 × 40, then `white-space: pre-wrap`, the box's width and height, `overflow-wrap: anywhere` for a word broken at a character and `normal` for one moved whole (`REF-MDN-OVERFLOW-WRAP`), `text-wrap: balance` for the balanced wrap (`REF-MDN-TEXT-WRAP`), the block placed down the box by its vertical alignment, and, for a maximum of 2 lines with the ellipsis on, `display: -webkit-box`, `-webkit-box-orient: vertical`, `-webkit-line-clamp: 2` and `overflow: hidden` (`REF-MDN-LINE-CLAMP`).
- Given a background, when written, then each paragraph is a `span` whose `background-color` is the background's color with its opacity, so that each line is highlighted as the library's bands are (`REF-CSS-INLINE-3`, §5.3).
- Given the HTML layer off, when written, then no `switch` nor `foreignObject` is written and every shape and text of E01, E14, F05, F14 and F16 is written as before.

## Tasks

One task = one commit, referenced as `EN-041.Tn`.

- [ ] T1 — A fitness test: every element `src/` and `playground/` create is one ADR-0053 allows, in its namespace, and a document written puts each in its place (`style` only in `defs`, `foreignObject` only as the first child of a `switch`, `switch` only around a text's group); mutation-checked (CHK-017's evolvability check) (1 h)
- [ ] T2 — The CSS of a text's options for its HTML layer, as plain data (property and value), `@see REF-MDN-WHITE-SPACE`, `REF-MDN-OVERFLOW-WRAP`, `REF-MDN-LINE-CLAMP`, `REF-MDN-TEXT-WRAP`, `REF-MDN-TEXT-DECORATION-THICKNESS`; tests first (3 h)
- [ ] T3 — Where the layer is placed: the block's or box's rectangle, raised by half the line gap for an automatic line height, from the first line's top before a trim, `@see DERIV-html-layer`; its condition (`USE_TYPO_METRICS` unset with `win` equal to `hhea`, or set with the typographic values equal to `hhea`; Arial and Inter 4.1 meet it) written in the TSDoc; tests first, the derivation's checks (2 h)
- [ ] T4 — The `switch` in the render: the `foreignObject` in the SVG namespace, its `div` and `span` in the XHTML namespace, the text's group second without its `transform`, an outer group carrying the turn; the option off by default, the output then exactly as before; `shapes.md` (HTML layer) updated; tests first (2 h)
