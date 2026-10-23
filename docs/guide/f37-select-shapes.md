# F37 — Select shapes

## What you can do now

**Select several shapes with Shift.** Click a shape, then click another with Shift held: both are selected, each outlined, and a dashed box surrounds the two. Shift + click on a shape already selected takes it out; with one left, the panel edits it again. A click without Shift, or with Ctrl, selects the shape clicked alone; Shift + click on an empty place changes nothing.

**The panel says how many.** With several shapes selected, the panel hides its fields and its title says how many are selected, "2 shapes selected": their position and size come with F38. A shape added with a tool goes on top of all and is selected alone, the others unchanged.

**Select all, or none, by a key.** Press Ctrl + A (Cmd + A on a Mac) to select every shape, and Escape to select none. With the cursor in a field of the panel, both keys stay the field's: Ctrl + A selects its text. With a color picker open, Escape closes the picker and leaves the selection as it is.

**Remove the selected shapes by a key.** Press Delete, or Backspace (the key labelled delete on a Mac): the shapes selected are removed, the others stay where they were, and none is selected. With the cursor in a field of the panel, Width or a text's Content, the key edits the field and no shape is removed. In Points mode the key deletes the points selected, as before, never the shape.

**Select by a rectangle.** With the Selector, press an empty place of the drawing and drag: a tinted rectangle follows the pointer. At the release, every shape it touches is selected, even by a corner of its outline; hold Alt (Option on a Mac) as you release to select only the shapes it holds whole, and Shift to add them to the shapes already selected. A round shape is touched by its outline, not by its box: a rectangle over the corner of its box, outside the circle, selects nothing. Pressing on a shape and dragging draws no rectangle and leaves the selection as it is: that gesture will move shapes, in a later feature.

**Points mode edits one shape.** While several shapes are selected, the Points button is disabled. In Points mode, Shift + click and a click on a point's handle act on the points only, never on which shapes are selected, and Ctrl + A selects no shape.

This page also says which keys the editor uses, chosen at the spike.

## The choices the agent made

For the review's summary; each can be changed.

- Shift + click on an empty place of the drawing keeps the selection, nothing being there to add or take out; a plain click there still selects none.
- The dashed box around several shapes is the smallest upright box holding their outlines as drawn, dashed as the straight box of a turned shape: 4 long every 7, 1 thick, inside the box.
- With several shapes selected, none is edited: the panel shows no field until F38. Back to one, the panel loads it as a click does, and the shape the panel left keeps its settings for the next shape of its kind.
- Points mode edits one shape: its button is disabled while several are selected, and in Points mode a click with Shift or on a handle never changes the shapes selected, nor does Ctrl + A. A plain click on a handle used to select none when the point lay in a rounded corner's cut; it now selects the point only.
- Escape selects none in Shape mode and in Points mode alike, as a click on an empty place does; in a field of the panel it changes nothing.
- Delete removes the selected shapes whichever tool is chosen, and the tool stays chosen; with no shape selected it does nothing. The shape the panel showed keeps its settings for the next shape of its kind: a rectangle of radius 25 removed, the next rectangle starts with radius 25. A shape with a refused value in the panel is removed too, as a click on an empty place drops it.
- In Points mode, Delete deletes the points selected and never the shape, even with no point selected. Every field of the panel keeps the key, a text's Content too, which used to let Delete reach the points.
- A selection rectangle that only meets a shape along its outline, a side or a corner, touches it, and a rectangle drawn exactly along a shape's sides encloses it.
- The selection rectangle snaps to whole units, as a drag with a tool does, and shows once the pointer has moved by one; it is a light tint with a thin outline of 1 pixel, and the outline of the shape under the pointer is hidden while you drag it.
- A drag with the Selector is never a click: pressing on a shape, dragging away and coming back, then releasing, selects nothing. Only a release before the pointer has moved away is a click.
- In Points mode a drag draws no rectangle and selects nothing, the mode editing one shape.
- Shift + a rectangle only adds shapes, never takes one out, even one already selected; a rectangle taking none leaves the selection as it is.
- A shape with a width or a height of 0 is a line: a selection rectangle touches it only by crossing that line, with no margin, though a click 2 pixels beside it picks it.

## The ideas behind it

**The keys are Figma's.** Each key the editor offers is, by default, the one Figma uses, so that someone who knows Figma finds them at once; every key can be changed later, in the settings (E20). Shift + click adds a shape to the selection, or takes it out; Ctrl + A (Cmd + A on a Mac) selects every shape; Escape selects none; the arrow keys will move the selection by 1, and by 10 with Shift; Ctrl + Z will undo. Ctrl + click is kept for picking a shape inside a group, as Figma does, which comes with E13: until then it selects the shape clicked alone (decided on 2026-10-23, replacing the rule of 2026-10-11 for the drawing; the list of points keeps its Ctrl + click).

**Where Figma's help says nothing, two other editors decide.** Figma's pages read do not name the keys for deleting and redoing: Delete or Backspace deletes, as in Inkscape and Penpot and as the points already do; Ctrl + Shift + Z redoes, as in Penpot and on a Mac, and Ctrl + Y too, as in Inkscape and on Windows. The agent chose these. To confirm at VAL-038.

**A key never reaches the drawing while you type.** With the cursor in a field of the panel, Delete, Backspace and Ctrl + A edit the field's text, never the shapes.

| Term | In plain words |
| --- | --- |
| selection | the shapes chosen, which the next action applies to |
| nudge | moving the selection by a small step with an arrow key |
| selection rectangle | a rectangle dragged on the drawing to select the shapes in it |
