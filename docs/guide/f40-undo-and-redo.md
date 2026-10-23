# F40 — Undo and redo

## What you can do now

Nothing new on the page yet: the editor's library now keeps the history of the drawing, which the next stories of this feature connect to the keys, the buttons and the panel.

**One step per finished change.** Each time a change of the drawing is finished, the drawing and the shapes selected are recorded. A drawing the same as before takes no step: selecting shapes, or typing a value back as it was, never fills the history. A drawing changed puts the one before it into the past, where Undo finds it, and forgets whatever had been undone, as every editor read does.

**Only what changed is kept.** A step keeps a new copy of the shapes it changed only; every other shape is shared with the step before it. A rectangle 100 × 50 is 614 characters as the page keeps it, 1 228 bytes: nudge it once in a drawing of two rectangles and the history holds that old rectangle, two references to the shapes of the step, 8 bytes each, and the index of the shape selected, 8 bytes more, 1 252 bytes in all, where a full copy of the drawing would take 2 482.

**The selection comes back with the step.** Undo gives back the shapes selected before the change, Redo those after it, so that an undo shows what it changed: with A and B drawn and B selected, delete B, then undo: B is back, selected; redo, and B is gone again, nothing selected.

**A memory size, and a few steps kept whatever their size.** The history keeps at most 100 MB, 100 000 000 bytes, by default, and always its last 5 steps, however large; the designer will set both (US-076). Past the size, the oldest steps go first, only as many as needed: a history of two nudges, 2 504 bytes, bounded at 2 000 bytes with 1 step kept, drops the first nudge and keeps the second, 1 252 bytes; with 5 steps kept, it keeps both.

## The choices the agent made

For the review's summary; each can be changed.

- The history keeps whole states of the drawing, sharing what did not change, rather than a list of commands (ADR-0095): one way of recording for every change of the page, none needing an undo of its own; the Product Owner left the choice to the agent, asking for the way that makes most sense for the project.
- A step keeps the selection as it was when the step was taken: selecting alone changes the selection kept with the present step without making one, so that Undo gives back the selection just before the change and Redo the one just after it, as Penpot does.
- Two drawings are the same when each shape has the same text, in the same order: a shape moved up or down in the order is a change, and so a step.
- The size counts the data, not the browser's memory, which no browser measures for a part of a page: 2 bytes a character of a shape's text, 8 bytes a reference to a shape or an index selected. The shapes of the drawing on the page are not counted, since they are there with or without a history.
- 100 MB by default, read from a view of up to about 1 000 symbols that must stay pleasant to use; 5 steps kept whatever their size, GIMP's default.
- The steps undone, waiting for Redo, count in the size, but the bound never drops them: the next change does.

## The ideas behind it

**States rather than commands.** Figma, Inkscape and Penpot record, for each action, what it changed, so that it can be put back; Redux and the Memento keep the document's states instead. A command must be written for each of the twenty places the page changes the drawing, and one forgotten leaves a change that cannot be undone; a state restored is exactly the drawing it was. Since the editor's data are never modified, only replaced, a state shares every shape it did not change, as Immer and Clojure share what a change did not reach.

**Measured from the data.** GIMP bounds its history by a memory size it adds up from the parts of what it keeps, and always keeps a few steps whatever their size; the editor does the same, counting a text by its characters, as JavaScript stores a string.

| Term | In plain words |
| --- | --- |
| step | one finished change of the drawing, that Undo takes back and Redo does again |
| state | the drawing and its shapes selected, as they were after a step |
| shared | the same shape kept once, by several states, instead of a copy in each |
