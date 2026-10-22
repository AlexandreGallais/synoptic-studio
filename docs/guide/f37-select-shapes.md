# F37 — Select shapes

## What you can do now

Nothing new yet: F37's stories come after its spike. This page says which keys the editor will use, chosen at the spike; the rest is written at F37's validation (VAL-038).

## The ideas behind it

**The keys are Figma's.** Each key the editor offers is, by default, the one Figma uses, so that someone who knows Figma finds them at once; every key can be changed later, in the settings (E20). Shift + click adds a shape to the selection, or takes it out; Ctrl + A (Cmd + A on a Mac) selects every shape; Escape selects none; the arrow keys will move the selection by 1, and by 10 with Shift; Ctrl + Z will undo. Ctrl + click adds or takes out a shape too, as asked on 2026-10-11, though Figma keeps it for picking a shape inside a group, which comes with E13.

**Where Figma's help says nothing, two other editors decide.** Figma's pages read do not name the keys for deleting and redoing: Delete or Backspace deletes, as in Inkscape and Penpot and as the points already do; Ctrl + Shift + Z redoes, as in Penpot and on a Mac, and Ctrl + Y too, as in Inkscape and on Windows. The agent chose these. To confirm at VAL-038.

**A key never reaches the drawing while you type.** With the cursor in a field of the panel, Delete, Backspace and Ctrl + A edit the field's text, never the shapes.

| Term | In plain words |
| --- | --- |
| selection | the shapes chosen, which the next action applies to |
| nudge | moving the selection by a small step with an arrow key |
| selection rectangle | a rectangle dragged on the drawing to select the shapes in it |
