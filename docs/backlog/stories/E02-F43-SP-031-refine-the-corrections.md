---
id: SP-031
epic: E02
feature: F43
title: Refine the corrections from the first try
status: ready
points: 2
---

# E02 · F43 · SP-031 — Refine the corrections from the first try

The spike that opens F43, its research done in full before its stories (Product Owner, 2026-10-22): the sources, the code each story will touch, every question at once, the feature's size known before any code.

## Acceptance criteria

- Given the corrections the Product Owner asked for on 2026-10-23 (E02's file, Remarks from the Product Owner's first try), when the spike ends, then each is traced to the playground's code it touches, what graphic editors do is read where the choice is not obvious, and F43's stories are written, 2 to 4 of at most 3 tasks each, criteria with numeric examples; a larger scope proposed as a split, with an example.
- Given what the sources and the code leave open, when the spike ends, then every question for the Product Owner is listed at once, each with options and a numeric example.

## The corrections asked

1. **The helps visible on any color**: the hover, the selection and the selection rectangle's outlines are blue, unseen on a blue shape; a double outline, dark and light, seen on every color (how Figma, Penpot and Inkscape draw them).
2. **The selection rectangle shows in advance** the shapes it would take, outlined as hovered, while it is dragged.
3. **A field showing Mixed** is stepped by the arrows as a number field (Up adds 1 to each shape, Shift + Up 10), so that several shapes' values move as quickly as one shape's.
4. **An arrow held in a number field** makes one history step once the value stops changing, not one per key repeat, as an arrow held on the drawing (SP-029's answer 3).
5. **Two arrows held at once** (Up and Right) move the selection diagonally.
6. **A text on the drawing is not selected as page text**: a press, a drag or a double click on it never highlights its characters (`user-select: none` on the drawing), the shape still picked.
7. **Shift while drawing a text's box** keeps it square, as every other tool's box.
8. **The history's list scrolls** when it holds more steps than it shows (AUD-044 fixed the History group split by the HTML parser: check whether it was the cause).
9. **Shift with the wheel**: panning across is a mouse's need; with a trackpad, whose two fingers already pan both ways, Shift should not force the pan across (how Figma tells a trackpad from a mouse, `REF-MDN-WHEEL-DELTAMODE`, Penpot's `normalize_wheel`).
10. **Width and height typed for several shapes** were seen not always to compute right: reproduce, fix if wrong.

Placed elsewhere, not in F43: the playground made responsive with a Figma-like toolbar at the bottom, icons for the selector, the shapes and the multiple selection (E20, the interface designed last); settings for the pan's speed and the zoom's strength, an expert menu (E20).

## Tasks

- [ ] T1 — The code read, the sources read, the questions for the Product Owner, F43's stories drafted (2 h)

One task = one commit, referenced as `SP-031.T1`.
