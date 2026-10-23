# F38 — The panel for several shapes

## What you can do now

**Read the position and size of several shapes.** Select two shapes or more, by Shift + click, Ctrl + A or a selection rectangle: under the panel's title, four fields show their X, Y, Width and Height. A value the shapes share is shown as it is; where they differ, the field is empty and reads Mixed, as the Radius field does for corners of different radii. Two rectangles, one 100 × 50 at (10, 20) and one 40 × 40 at (200, 20), show X Mixed, Y 20, Width Mixed and Height Mixed: they sit on one line, and differ in everything else.

**A turned shape counts by its straight box.** X and Y are the top-left corner of the upright box around the shape as it is drawn, as the panel shows them for one shape; Width and Height are the shape's own, before it turns. The rectangle 100 × 50 at (10, 20), turned by 30°, has X 4.19873 and Y −1.65064, Width 100 and Height 50.

**A text counts by its frame.** Its frame is its block, the rectangle its lines fill, or its box when it sits in one, as when the pointer picks it: X and Y are the top-left corner of the upright box around that frame, Width and Height the frame's own.

**The fields only show, for now.** They are read-only: typing a value for several shapes, added to each or set on each, comes with the next story of this feature. Click one shape alone and the panel shows and edits it as before, the four fields gone.

## The choices the agent made

For the review's summary; each can be changed.

- Two values count as one when they differ by no more than a billionth of a unit, the editor's tolerance for computed numbers, so that a side computed a hair off zero still reads 0 beside a side at 0; the first shape's value is shown, with the five decimals of the SVG.
- The values of several shapes have four fields of their own, X, Y, Width and Height, the shape's other fields staying hidden, so that the settings a new shape takes from the last one of its kind are never overwritten by a selection of several; they are read-only rather than greyed out until the typing comes.
- A text never measured, which is not drawn, is left out of the values, as it is left out of the dashed box; with nothing left to show, each field is empty, without Mixed.

## The ideas behind it

**Each shape's values, as Figma shows them.** Figma and Penpot show the values of several shapes field by field, a value common to them as it is and Mixed where they differ; Inkscape and Affinity Designer show the box around the whole selection instead. The first way was chosen on 2026-10-23, at the refinement of this feature, so that you see at once whether the shapes line up and match.

| Term | In plain words |
| --- | --- |
| Mixed | what a field reads when the shapes selected have different values for it |
| straight box | the smallest upright rectangle around a shape as it is drawn, turned or not |
| frame | the rectangle a text is laid out in: its block, or its box when it sits in one |
