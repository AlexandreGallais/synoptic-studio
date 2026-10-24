# F38 — The panel for several shapes

## What you can do now

**Read the position and size of several shapes.** Select two shapes or more, by Shift + click, Ctrl + A or a selection rectangle: under the panel's title, four fields show their X, Y, Width and Height. A value the shapes share is shown as it is; where they differ, the field is empty and reads Mixed, as the Radius field does for corners of different radii. Two rectangles, one 100 × 50 at (10, 20) and one 40 × 40 at (200, 20), show X Mixed, Y 20, Width Mixed and Height Mixed: they sit on one line, and differ in everything else.

**A turned shape counts by its straight box.** X and Y are the top-left corner of the upright box around the shape as it is drawn, as the panel shows them for one shape; Width and Height are the shape's own, before it turns. The rectangle 100 × 50 at (10, 20), turned by 30°, has X 4.19873 and Y −1.65064, Width 100 and Height 50.

**A text counts by its frame.** Its frame is its block, the rectangle its lines fill, or its box when it sits in one, as when the pointer picks it: X and Y are the top-left corner of the upright box around that frame, Width and Height the frame's own. A text selected alone shows the same X and Y: a box 100 × 20 at (47, 61), turned by 23°, shows X 47.06745 and Y 42.2584, alone or among several, and 60 typed in X moves its upright box to start at x 60. A shape 0 × 0, alone or among several, shows the point it sits on.

**Type a value for all of them.** Type a whole number in one of the four fields and press Enter, or leave the field: with a sign, `+50` or `-20`, it is added to each shape's own value; without one, each shape takes it. The two rectangles above: `+50` in Width makes the first 150 wide and the second 90; `-20` in X moves them to x −10 and 180; 460 in Width makes both 460 wide, and 50 in X puts both at x 50. A negative number has a sign, so it always moves or shrinks the shapes from where they are. Click one shape alone and the panel shows and edits it as before, the four fields gone.

**Step them by the arrows.** Since F43, Up in one of the four fields adds 1 to each shape's own value, Down takes 1 away, and with Shift both go by 10, as `+1` or `-10` typed: the two rectangles above, Up in Width, are 101 and 41 wide, Width still reading Mixed; Up in Y moves both 1 down, the way a number field counts up, and Up in X 1 right, the arrows kept in X and Y as in Width and Height by the Product Owner's choice of 2026-10-24. A step that one shape cannot take changes none, as a value typed: a shape 0 high keeps both shapes as they are on Down in Height.

**Each shape keeps its own way of moving and growing.** X and Y place each shape's straight box, as the panel does for one shape: the turned rectangle above, typed 50 in X, has its upright box start at x 50. Width and Height set each shape's own size, a turned shape keeping its turned top-left corner in place, as when you type its width alone. A text in a box gets the new size for its box; a text fitting its content keeps its size, since it has no width of its own to set, and moves with the others.

**One shape's fields follow the same rule.** Since F44, the X, Y, Width and Height of a shape selected alone, and a text's X, Y and its box's Width and Height, answer the keyboard as the four fields of several shapes: Up adds 1, Shift + Up 10, and a value typed, `+10` or `150`, is applied when you press Enter or leave the field, never while you type it. The rectangle the page opens with, 120 × 80, is 130 wide for Shift + Up in Width, and moves to x 10 for Shift + Up in X; `+19` typed in Width and Enter makes it 19 wider at Enter only. Holding Up in Height is one step, which one Ctrl + Z takes back. A turned shape keeps its turned top-left corner, as with several.

**Every number field the same.** Since US-087, every other number field of the page answers the keyboard the same way: the radius, a polygon's corners, the angle, a text's size, opacities, spacings, lines and line height, a point's x and y and its handles, the color picker's H, S and B or L, the zoom's percentage, and the history's Memory and Steps kept. Up adds 1, Shift + Up 10, each field stepping by 1; `+5` typed and Enter adds 5, `-5` takes 5 away; since AUD-046 a value past the field's bounds is taken at the bound, an angle and the picker's hue going round, a fraction is rounded, and only a text that is no number is refused, the field showing its number again: the radius 10 is 20 after Shift + Up, 15 after `+5` and Enter, 0 for `-15` and 2 for `1.5`; 13 in Corners gives 12, 370 in Angle 10. In a radius showing Mixed, `+5` adds 5 to each corner's own radius. Since AUD-047 these fields always give a whole number, a fraction they show included: a handle's end at 11.83476 is 13 after Up, where before the handle did not move; the shapes' X, Y, Width and Height keep a shape's own fraction when a whole number is typed. The picker's H, S and B or L stepped by 0.1 before, which the picker rounded away, so that Up changed nothing.

**A value beyond a range is taken at its bound.** A size that would fall below 0 for a shape is 0 for that one, the others set as typed: `-50` in Width makes the first rectangle 50 wide and the second, 40 wide, 0 wide. A fraction is rounded once the value is computed, `+1.5` on 10 giving 12; only a word changes nothing, the fields showing the shapes' values again. Before AUD-046, a size below 0 for one shape, or a fraction, changed none of them.

## The choices the agent made

For the review's summary; each can be changed.

- Two values count as one when they differ by no more than a billionth of a unit, the editor's tolerance for computed numbers, so that a side computed a hair off zero still reads 0 beside a side at 0; the first shape's value is shown, with the five decimals of the SVG.
- The values of several shapes have four fields of their own, X, Y, Width and Height, the shape's other fields staying hidden, so that the settings a new shape takes from the last one of its kind are never overwritten by a selection of several.
- A value typed is applied on Enter or when the field is left, as the fields under the list of points, never while it is typed, so that a sign or an emptied field moves nothing on its way.
- An arrow in one of the four fields is applied at once, the fields then showing the shapes' values again: a text typed there and not yet applied by Enter is dropped, the other way being to apply it first. With Ctrl, Cmd or Alt held, the arrow is left to the field, which moves its caret.
- A size that would fall below 0 for a shape is taken at 0 for it, as every number field takes a value at its bound (the rule of 2026-10-24); before, it refused the value for all of them, so that a value went to every shape or to none. A size above 100 000 is brought down to 100 000, as the Width field of one shape does.
- A text never measured, which is not drawn, is left as it is when a value is typed; a shape 0 × 0, which is a point, is placed by that point.
- A text never measured, which is not drawn, is left out of the values, as it is left out of the dashed box; with nothing left to show, each field is empty, without Mixed.
- One shape's X, Y, Width and Height, and a text's box, are the fields the correction of 2026-10-24 named, which several shapes have too; each change is the computation several shapes use, from the shape as last drawn. A box's width above 100 000 is now brought down to 100 000, as a shape's, where it was refused. While a shape is refused, a value typed in its fields is written as typed, a value with a sign from 0.

## The ideas behind it

**Each shape's values, as Figma shows them.** Figma and Penpot show the values of several shapes field by field, a value common to them as it is and Mixed where they differ; Inkscape and Affinity Designer show the box around the whole selection instead. The first way was chosen on 2026-10-23, at the refinement of this feature, so that you see at once whether the shapes line up and match.

**A text placed as a shape is.** Until this feature, a turned text alone showed the top-left corner of its turned frame in X and Y; on 2026-10-23, at this feature's audit, X and Y of a text became the top-left of its upright box, as a shape's, alone or among several, so that one text and several read the same. Figma's help, read the same day, bases a rotated layer's X and Y "on the original top-left corner of the layer's bounds", the turned corner a text showed before.

**A sign for relative, none for fixed.** Figma applies a value typed to every layer selected, and adds to each layer's own with an equation such as `Mixed+100`; here a sign alone is enough, `+100`, chosen on 2026-10-23 with the rest of this feature, while a text fitting its content keeps its size, as it has no width to set.

**The arrows step each shape.** Penpot's fields step by Up and Down, 10 with Shift, but a field of several values goes to one value for all of them; on 2026-10-23, after the first try of the editor, each shape's own value was chosen to move instead, as a value typed with a sign moves it.

| Term | In plain words |
| --- | --- |
| Mixed | what a field reads when the shapes selected have different values for it |
| straight box | the smallest upright rectangle around a shape as it is drawn, turned or not |
| frame | the rectangle a text is laid out in: its block, or its box when it sits in one |
| relative value | a value typed with a sign, `+50` or `-20`, added to each shape's own rather than set on each |
