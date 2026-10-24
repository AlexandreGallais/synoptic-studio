# F42 — Zoom and pan

## What you can do now

**Zoom about the pointer.** Hold `Ctrl` (`Cmd` on a Mac) and turn the wheel over the drawing, or pinch on a trackpad: the drawing grows or shrinks about the pointer, the point under it staying under it. A turn of 100 pixels away from you zooms to 157 %; the same turn back gives 100 % again. The page itself never zooms: only the drawing does, from 1 % to 25 600 %.

**Move the view.** Turn the wheel alone and the view moves up or down; with `Shift`, left or right; two fingers on a trackpad move it every way. Hold `Space` and drag, or drag with the middle button, and the drawing follows the pointer: nothing is selected, moved nor placed by it. A space typed in a field of the panel stays the field's. Leaving the window lets `Space` go, even held, so that a drag back in the page does not pan by itself.

**No shape changes.** Zooming and moving the view change no number: a rectangle 100 × 50 at (10, 20) reads X 10, Y 20, W 100, H 50 at 200 % as at 100 %. The view is not a step of the history, so `Ctrl` + `Z` undoes the last change of the drawing and leaves the view as it is. **Export SVG** writes the same file at any zoom, in the frame the page opens with. **Reset** and each step of the guided test bring back that view.

**Work at any zoom.** A click places a shape where it is clicked, at any zoom; the handles, the frames, the dashed boxes and the outlines keep their size on screen, a handle's radius 8 pixels whether zoomed in or out. A press released within 3 pixels of where it was pressed is a click, at any zoom: zoomed out to 10 %, a hand that trembles by a pixel still places a square 100 × 100 rather than a thin line; with the Selector, a shape moves only once the pointer goes further than 3 pixels.

**Large shapes stay fast.** The outlines of the shape under the pointer and of the selection are computed only where they can be seen: the part of the drawing the canvas shows and half the canvas more on each side. A rectangle 99 999 × 100 000 rounded to the full, selected at 100 %, used to take a fifth of a second to outline at each redraw, its corners far out of sight; its outline is now empty there and the page stays smooth. Move the view by less than half the canvas and the outlines are not computed again; further, or when the drawing changes or the zoom does, they are.

**Zoom by keys.** Press `+` to zoom in and `-` to zoom out, one step at a time: 100, 141, 200, 283, 400 %, each step × √2. With the pointer over the drawing, the point under it stays under it; elsewhere, the zoom keeps the canvas's centre. `Shift` + `0` goes back to 100 % about the centre. A key typed in a field of the panel stays the field's; with `Ctrl` held, `+` and `-` zoom the page itself, as in any web page.

**See the whole drawing, or the selection.** `Shift` + `1` fits every shape in the canvas, 40 pixels left on each side; `Shift` + `2` fits the shapes selected. Three rectangles 100 × 50 spread from (10, 20) to (410, 220) fill a canvas of 1 200 × 800 at 280 %; one of them alone at 1 120 %. With no shape, or none selected, nothing changes.

**Read the zoom.** The toolbar shows the zoom, rounded to a whole percent: 141 % after one step in. Click it for its menu: Zoom in, Zoom out, Zoom to 100 %, Fit the drawing and Fit the selection, each with its key, and a field where a percentage is typed, applied by `Enter`: 200 gives 200 %, about the canvas's centre. A percentage beyond 1 % to 25 600 % is taken at the bound; a text, 0 or a number below 0 changes nothing.

**The view follows the work area's size.** Since F44, hiding the pipeline column, showing the checklist or resizing the window draws the page again at the new size: the drawing never shrinks nor moves, the outline of the shape under the pointer stays on it, and a click lands where it is seen. The view keeps its zoom and the point at its top-left corner, the canvas showing more or less to the right and at the bottom.

## The choices the agent made

For the review's summary; each can be changed.

- With `Shift`, outside macOS, the wheel moves the view across by its vertical turn and its horizontal one, added. Since F43, on macOS, which already turns a mouse's `Shift` + wheel across, the view follows the wheel's turns as sent, so that two fingers swiping diagonally on a trackpad with `Shift` move it both ways, as in Penpot.
- The system is told as the page opens by the platform the browser names, `MacIntel` on a Mac; the browsers' newer way of naming it is not yet in all of them.
- The wheel's vertical turns sent within one frame of the screen zoom once by their sum, so that a slow pinch zooms slowly and a turn back gives the same zoom back. Penpot zooms by the same amount a pixel, but adds each turn's horizontal part to its vertical one and multiplies the zooms of a frame: two turns of 50 pixels zoom here by 1.57, in Penpot by 1.65.
- Shapes dragged while the wheel zooms stay under the pointer: their place is read through the new view at each move of the pointer.
- `=` zooms in as `+` does, as in Inkscape, so that `+` needs no `Shift` on a US keyboard.
- `Shift` with `0`, `1` or `2` is read by the key's place on the keyboard, so that it works whatever character the layout gives it: `)` on a US keyboard, `0` on a French one.
- The menu's zooms keep the canvas's centre, the pointer being on the menu; a percentage typed beyond the bounds is taken at the bound, and a text, 0 or a number below 0 is refused, the field showing the zoom again.
- An outline is cut to the region it is computed for by its pieces' chords, each drawn whole or left out, rather than by an arc's angles and a curve's halves, so that the outline seen is exactly the one drawn before, every chord the same.

## The ideas behind it

**A window on the drawing.** The drawing's coordinates never change; what changes is the part of it the canvas shows, the SVG's `viewBox`: its top-left corner, the **origin**, and its size, the canvas's size divided by the **scale**. At 200 % with (180, 130) at the canvas's top-left, a canvas of 1 200 × 800 pixels shows the region from (180, 130), 600 × 400 units. Both layers, the drawing and the outlines above it, take the same `viewBox`, so that they stay aligned.

**Zoom about a point.** The point under the pointer is the origin plus the pointer's place on the canvas divided by the scale. To keep it under the pointer at a new scale, the origin moves by the difference: from 100 % to 200 % with the pointer at (440, 340) over (400, 300), the origin becomes (400 − 440 / 2, 300 − 340 / 2) = (180, 130).

**A fit.** To show a box whole with 40 pixels on each side, the scale is the smaller of the room across and down divided by the box's width and height, and the box's centre goes to the canvas's centre: the box from (10, 20) to (410, 220) in a canvas of 1 200 × 800 gets min(1 120 / 400, 720 / 200) = 2.8, its centre (210, 120) at the offset (600, 400), so the origin is (210 − 600 / 2.8, 120 − 400 / 2.8) = (−4.29, −22.86).

**Sizes in screen pixels.** A help drawn in the drawing's units would grow with the zoom: a handle of radius 8 would be 2 048 pixels across at 25 600 %. Each help is given in screen pixels and divided by the scale, so that it keeps its size on screen.

**Only what can be seen.** An outline follows a curve by short straight chords, 2 pixels long on screen. Each chord is drawn or left out whole: kept when it comes within the outline's half width of the region computed, the area shown and half the canvas around it (`DERIV-band-within-box`). A whole run of chords far from that region is left out by testing one point in its middle: none of its chords can be farther from that point than half the run's length.

| Term | In plain words |
| --- | --- |
| view | how much the drawing is zoomed and which part of it the canvas shows; never saved in the drawing |
| scale | screen pixels per unit of the drawing: 1 at 100 %, 2 at 200 % |
| origin | the point of the drawing at the canvas's top-left corner |
| `viewBox` | the region of the drawing an SVG shows, in the drawing's units |
| pan | moving the view, the shapes staying where they are |
| fit | the zoom and the view that show a box whole, centred, 40 pixels from the canvas's sides |

## Limits

The keys are not editable yet (E20); the view is not kept once the page is closed; there is no Hand tool in the toolbar, `Space` and the middle button doing its work.
