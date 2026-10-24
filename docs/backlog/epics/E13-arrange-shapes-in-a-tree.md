---
id: E13
title: Arrange shapes in a tree and lay them out
status: dropped
---

# E13 — Arrange shapes in a tree and lay them out

**Dropped on 2026-10-25** (SP-033, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md) §4): its content went to [E27](./E27-group-shapes-and-order-their-stacking.md), the shape tree, [E28](./E28-lay-out-shapes-in-boxes.md), the layout box, and [E44](./E44-edit-the-shape-tree-in-a-layers-panel.md), the layers panel.

Idea of the Product Owner (2026-10-10, refining F07): an epic between the shapes and the symbols, about the tree only, so that shapes can hold other shapes, in the code at least, and the layout box (Q21) can place them.

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | need to place shapes and text inside a box, in line with each other, without computing every position |
| the solution | a shape tree and a layout box |
| is a | a tree of nodes in the model; a rectangular box that places its children by anchor, or stacks them in a row or a column with gaps and padding |
| that | every child keeps its integer numbers; its position is derived from the box |
| unlike | Figma, where a child leaving the auto layout (absolute position) no longer follows its parent |
| our solution | a child may override the box's anchor, fixed bottom left for example, and stay laid out by the box |
| Business outcomes | a box holding a polygon, a rectangle and a text places them by its anchor, one child overriding it, and the result stays editable |
| Leading indicators | Q21 settled; tree in the model; layout reference cases accepted |
| Non-functional requirements | derived positions never written back into the model (ADR-0003); integer inputs only |
| In scope | shape tree in the model (groups, children, stacking order), layout box, anchor per child, row and column stacking with gaps, padding |
| Out of scope | layers panel (rename, hide, lock: `interaction.md` §6, end of project); a box resizing to its children (Figma's hug); a non-rectangular box |
| Closure criteria | the layout cases of `shapes.md` §3 render as specified in the playground |

## Candidate features

| Feature | Status |
| --- | --- |
| Shape tree in the model | to refine |
| Layout box with an anchor | to refine |
| Child overriding the anchor | to refine |
| Row and column stacking, gap, padding | to refine |

## To refine

- **Name**: "layout" is Figma's word; keep it only if it is the best term, otherwise find another (Product Owner): to be sourced at the epic's first spike.
- **Order** (Product Owner, 2026-10-10, kept on 2026-10-12): before E03, because its non-destructive booleans are nodes of the tree (ADR-0006); E05's symbols are then groups of this tree.
- **Settled (Product Owner, 2026-10-10)**: a child given its own anchor stays in the box but leaves the stack completely; the stack keeps the box's layout. Stacks only exist while the symbol is edited: once used, every position is fixed (see `shapes.md` §3, Q21).
- F07 (the polygon anchored in its own box) is the first, single-child case: E13 reuses its 3 × 3 anchor.
