# F38 — The panel for several shapes

## What you can do now

**Read the position and size of several shapes.** Select two shapes or more, by Shift + click, Ctrl + A or a selection rectangle: under the panel's title, four fields show their X, Y, Width and Height. A value the shapes share is shown as it is; where they differ, the field is empty and reads Mixed, as the Radius field does for corners of different radii. Two rectangles, one 100 × 50 at (10, 20) and one 40 × 40 at (200, 20), show X Mixed, Y 20, Width Mixed and Height Mixed: they sit on one line, and differ in everything else.

**A turned shape counts by its straight box.** X and Y are the top-left corner of the upright box around the shape as it is drawn, as the panel shows them for one shape; Width and Height are the shape's own, before it turns. The rectangle 100 × 50 at (10, 20), turned by 30°, has X 4.19873 and Y −1.65064, Width 100 and Height 50.

**A text counts by its frame.** Its frame is its block, the rectangle its lines fill, or its box when it sits in one, as when the pointer picks it: X and Y are the top-left corner of the upright box around that frame, Width and Height the frame's own.

**Type a value for all of them.** Type a whole number in one of the four fields and press Enter, or leave the field: with a sign, `+50` or `-20`, it is added to each shape's own value; without one, each shape takes it. The two rectangles above: `+50` in Width makes the first 150 wide and the second 90; `-20` in X moves them to x −10 and 180; 460 in Width makes both 460 wide, and 50 in X puts both at x 50. A negative number has a sign, so it always moves or shrinks the shapes from where they are. Click one shape alone and the panel shows and edits it as before, the four fields gone.

**Each shape keeps its own way of moving and growing.** X and Y place each shape's straight box, as the panel does for one shape: the turned rectangle above, typed 50 in X, has its upright box start at x 50. Width and Height set each shape's own size, a turned shape keeping its turned top-left corner in place, as when you type its width alone. A text in a box gets the new size for its box; a text fitting its content keeps its size, since it has no width of its own to set, and moves with the others.

**A value that does not fit changes nothing.** A fraction, a word, or a size that would fall below 0 for one shape changes none of them, and the fields show the shapes' values again: `-50` in Width leaves both rectangles as they were, since the second, 40 wide, cannot be −10 wide.

## The choices the agent made

For the review's summary; each can be changed.

- Two values count as one when they differ by no more than a billionth of a unit, the editor's tolerance for computed numbers, so that a side computed a hair off zero still reads 0 beside a side at 0; the first shape's value is shown, with the five decimals of the SVG.
- The values of several shapes have four fields of their own, X, Y, Width and Height, the shape's other fields staying hidden, so that the settings a new shape takes from the last one of its kind are never overwritten by a selection of several.
- A value typed is applied on Enter or when the field is left, as the fields under the list of points, never while it is typed, so that a sign or an emptied field moves nothing on its way.
- A size that would fall below 0 for one shape refuses the value for all of them, so that a value goes to every shape or to none; the other way was to change the shapes that can take it. A size above 100 000 is brought down to 100 000, as the Width field of one shape does.
- A text never measured, which is not drawn, is left as it is when a value is typed; a shape 0 × 0, which is a point, is placed by that point.
- A text never measured, which is not drawn, is left out of the values, as it is left out of the dashed box; with nothing left to show, each field is empty, without Mixed.

## The ideas behind it

**Each shape's values, as Figma shows them.** Figma and Penpot show the values of several shapes field by field, a value common to them as it is and Mixed where they differ; Inkscape and Affinity Designer show the box around the whole selection instead. The first way was chosen on 2026-10-23, at the refinement of this feature, so that you see at once whether the shapes line up and match.

**A sign for relative, none for fixed.** Figma applies a value typed to every layer selected, and adds to each layer's own with an equation such as `Mixed+100`; here a sign alone is enough, `+100`, chosen on 2026-10-23 with the rest of this feature, while a text fitting its content keeps its size, as it has no width to set.

| Term | In plain words |
| --- | --- |
| Mixed | what a field reads when the shapes selected have different values for it |
| straight box | the smallest upright rectangle around a shape as it is drawn, turned or not |
| frame | the rectangle a text is laid out in: its block, or its box when it sits in one |
| relative value | a value typed with a sign, `+50` or `-20`, added to each shape's own rather than set on each |
