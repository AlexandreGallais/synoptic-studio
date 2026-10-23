# F41 — The list of the history's steps and a history kept fast

## What you can do now

**See the last steps and go back to one.** Click **Steps** in the toolbar, after Undo and Redo: a list opens under it, the oldest step first, each named by what it did. Place a rectangle, nudge the first one with Right, delete the new one, then type 150 in W and press Enter: the list reads `Start`, `Add`, `Move`, `Delete`, `Width`, and `Width` is marked, since it is where the drawing stands. `Start` is the drawing as the history first kept it.

**A click goes back, or forward.** Click `Move`: the drawing is as it was just after the nudge, the deleted rectangle back and the first one at its width before. `Delete` and `Width` stay in the list, greyed and in italics, waiting for Redo; click `Width` and the drawing comes back to it in one go, as many Redo at once. The shapes selected come back with each step, as with Undo.

**A change after going back drops the steps waiting.** Click `Move`, then nudge the rectangle again: `Delete` and `Width` are gone from the list, and Redo is greyed out, as after any Undo followed by a change.

**A long history stays a short list.** The list shows at most 15 steps: the one marked, up to 7 after it, and the steps before it. Those left out are counted on a line, `9 earlier steps` above the list, `6 later steps` below it. Nudge the rectangle 20 times after the steps above, and the list shows the last 15, under `9 earlier steps`.

**A step stays instant with a full history.** The history keeps its memory size up to date at each step instead of adding it all up again: with 80 000 moves held, about what 100 MB holds, a step takes about 2 ms, where it took 63 ms before. Showing the list and going back 14 steps take about 0.06 ms and 1 ms with that history.

## How each step is named

The history keeps the drawing as it was after each step, not the actions themselves (ADR-0095). A step is named by comparing the drawing before it with the drawing after it:

| The drawing after the step | Its name |
| --- | --- |
| more shapes, every other one unchanged and in the same order | `Add` |
| fewer shapes, every other one unchanged and in the same order | `Delete` |
| as many shapes, only their place changed | `Move` |
| as many shapes, the kind shown in the panel changed, a polygon made a free shape | `Convert` |
| as many shapes, one setting changed besides the place | that setting in words: `Width`, `Height`, `Angle`, `Content`, `Box width` |
| anything else, two settings changed at once among them | `Change` |

So a width typed that also moves the shape, its place following its anchor, is still `Width`; a text box's width and height reset together are `Change`. The words are those of the editor's settings, `Corners` for a polygon's number of corners as for the points of a free shape.

## The choices the agent made

For the review's summary; each can be changed.

- The names are read from the drawings before and after each step rather than written by each of the twenty places that change the drawing, so that none can be forgotten; a setting's name is its name in the editor's data, split into words.
- The oldest step kept is `Start`, even after the memory dropped older ones.
- At most 15 steps are shown, at most 7 of them after the one marked, so that the list costs the same with 80 000 steps as with 10; the steps left out are counted rather than scrolled to.
- A click on a step is as many Undo or Redo at once: what the page shows is recorded first, as for Undo, and the steps after it are kept for Redo.
- The list opens from **Steps** in the toolbar and stays open until it is clicked again; the step marked is the drawing shown, the steps for Redo greyed and in italics.
- The memory used is kept up to date at each step: each shape the history holds counts the steps that hold it, kept so that each step copies only what it changes (ADR-0097).

## The ideas behind it

**A list of states.** GIMP's Undo History lists the actions most recently performed and goes back to any of them by a click, back and forth without losing anything. The editor's history keeps states, each sharing the shapes it did not change, so going to a step is only moving the marker: Undo as many times as needed, or Redo.

**Counting as you go.** Adding up the history's memory at each step costs more as the history grows. Keeping the count with the history, and changing it only by the shapes a step adds or drops, makes a step cost what it changes, whatever the history holds.

| Term | In plain words |
| --- | --- |
| step | one finished change of the drawing, that Undo takes back and Redo does again |
| state | the drawing and its shapes selected, as they were after a step |
| present | the state the page shows, marked in the list |
| steps to redo | the steps after the one marked, kept until the next change |
