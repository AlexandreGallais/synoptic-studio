# F40 — Undo and redo

## What you can do now

**Undo and Redo on the drawing.** Place a shape, draw one, move shapes, delete them or nudge them with the arrows, then press Ctrl + Z, Cmd + Z on a Mac, or click Undo in the toolbar: the drawing goes back one change. Ctrl + Shift + Z or Ctrl + Y, or Redo, does it again. Each button is greyed out while there is nothing to undo or redo, and beside them the toolbar shows how much memory the history holds: `History 1.2 kB of 100 MB` once the rectangle the page opens with is nudged.

**A gesture is one step.** A drag is one step when the button is released, whatever its path; a drag the browser cancels takes none. An arrow held down is one step from the moment it is pressed to the moment it is released, however many times the browser repeats it: hold Right for a second, the shape goes 30 to the right, and one Ctrl + Z brings it all the way back.

**What is typed and chosen in the panel.** A value is one step once applied, by Enter or by leaving its field, however many characters were typed: type 150 in W over 100, press Enter, and one Ctrl + Z brings the shape back to 100, though it was drawn 1, then 15, then 150 wide. Each choice and each button of the panel is a step too — the anchor, Stretched, a turn, Convert to free shape, the points added or deleted, the handles, a text's weight, Bold, Italic, its box —, for one shape or several. A value typed back as it was takes no step. In the color picker, each key on a slider is a step, a drag on the plane is one when released, and a color typed is one when applied: open the picker, press Right five times on the hue slider, drag the plane, then type a color, and seven Ctrl + Z walk back through them.

**A text placed is one step with what you type in it.** Choose the Text tool and click the drawing: the text starts empty, a thin dashed bar one line tall showing where its first letter will go, and the Content field has the focus. Type `Valve` and leave the field: placing the text and typing it are one step, and one Ctrl + Z removes the text, since nothing was there before. Its size or its font may be set before typing it, in the same step. A text left empty is removed as soon as you click elsewhere, choose another tool or press Escape, and takes no step.

**A value refused takes no step.** Type 13 in a polygon's Corners, refused since a polygon has 3 to 12, then 8: that is one step, from 6 to 8. Ctrl + Z while the value is refused gives back the shape as it was last drawn; a click on the empty drawing drops the shape refused, and Ctrl + Z brings it back.

**In a field of the panel.** While you type in a field, Ctrl + Z is the field's own, as in any text box. Once the value is applied, by Enter or by leaving the field, Ctrl + Z goes to the drawing even if the field still has the focus: type 150 in W, press Enter, then Ctrl + Z, and the shape is 100 wide again.

**How much the history keeps is yours to set.** Beside Undo and Redo, Memory is the most the history holds, in MB, 100 when the page first opens, and Steps kept the number of last steps kept whatever their size, 5; hover over either for what it does. The toolbar shows how many moves of one shape the memory holds about, `about 80,380 moves` for 100 MB, a move adding about 1 244 bytes. Type a value and press Enter, or leave the field: the history fits it at once, its oldest steps going first, never the steps waiting for Redo, and the browser remembers both values for the next time the page is opened. Set Memory to 0 and Steps kept to 2, nudge the rectangle four times, and only the last two nudges can be undone. Memory takes a whole number from 0 to 1 000 and Steps kept from 0 to 100; anything else is put back as it was.

**Reset and the guided test start a new history**, with nothing to undo, as the page opens; the memory and the steps kept stay as set.

The history itself, kept by the editor's library:

**One step per finished change.** Each time a change of the drawing is finished, the drawing and the shapes selected are recorded. A drawing the same as before takes no step: selecting shapes, or typing a value back as it was, never fills the history. A drawing changed puts the one before it into the past, where Undo finds it, and forgets whatever had been undone, as every editor read does.

**Only what changed is kept.** A step keeps a new copy of the shapes it changed only; every other shape is shared with the step before it. A rectangle 100 × 50 is 614 characters as the page keeps it, 1 228 bytes: nudge it once in a drawing of two rectangles and the history holds that old rectangle, two references to the shapes of the step, 8 bytes each, and the index of the shape selected, 8 bytes more, 1 252 bytes in all, where a full copy of the drawing would take 2 482.

**The selection comes back with the step.** Undo gives back the shapes selected before the change, Redo those after it, so that an undo shows what it changed: with A and B drawn and B selected, delete B, then undo: B is back, selected; redo, and B is gone again, nothing selected.

**A memory size, and a few steps kept whatever their size.** The history keeps at most 100 MB, 100 000 000 bytes, by default, and always its last 5 steps, however large; the designer sets both beside Undo and Redo. Past the size, the oldest steps go first, only as many as needed: a history of two nudges, 2 504 bytes, bounded at 2 000 bytes with 1 step kept, drops the first nudge and keeps the second, 1 252 bytes; with 5 steps kept, it keeps both.

## The choices the agent made

For the review's summary; each can be changed.

- The history keeps whole states of the drawing, sharing what did not change, rather than a list of commands (ADR-0095): one way of recording for every change of the page, none needing an undo of its own; the Product Owner left the choice to the agent, asking for the way that makes most sense for the project.
- A step keeps the selection as it was when the step was taken: selecting alone changes the selection kept with the present step without making one, so that Undo gives back the selection just before the change and Redo the one just after it, as Penpot does.
- Two drawings are the same when each shape has the same text, in the same order: a shape moved up or down in the order is a change, and so a step.
- The size counts the data, not the browser's memory, which no browser measures for a part of a page: 2 bytes a character of a shape's text, 8 bytes a reference to a shape or an index selected. The shapes of the drawing on the page are not counted, since they are there with or without a history.
- 100 MB by default, read from a view of up to about 1 000 symbols that must stay pleasant to use; 5 steps kept whatever their size, GIMP's default.
- The steps undone, waiting for Redo, count in the size, but the bound never drops them: the next change does.
- On the page, a step is taken at each click on the drawing, each release of the pointer and each key released outside the panel's fields, once the page is redrawn; while a shape is being moved or drawn, the keys of the history wait for the release, as Delete and the arrows do.
- Undo first records the page as it is, so that a change not yet recorded is what Ctrl + Z takes back.
- A step of the panel is taken from the page as its field drew it, so that a value a field refuses stays in it as typed.
- No step while the shape edited is refused; Ctrl + Z then gives back its last valid values without undoing anything else, and the shape dropped by a click elsewhere is a step Ctrl + Z undoes.
- A new text starts empty, as the Product Owner asked: its frame, while empty, is 1 wide and one line tall, centred on the point clicked, so that the first line is centred there and the text grows to the right; an empty text not being typed still shows no frame.
- A new text left empty is removed when the panel leaves it, rather than kept invisible; leaving its Content field empty keeps it, so that its size can be set first.
- Ctrl + Z while a new text is still empty removes it without undoing anything else.
- A drag of a shape not selected gives back, undone, the selection before it, none, the press alone selecting nothing; a tool armed stays armed after an undo; Ctrl + Z is taken even with nothing to undo, so the browser never acts on it, and left to the browser with Alt held.
- The size is written in B, kB, MB or GB of 1 000 each, to one decimal, `1.2 kB`.
- Memory is set in whole MB from 0 to 1 000 and Steps kept from 0 to 100, so that 100 steps of 1 000 rectangles changed at once, about 124 MB, stay near the default size; a value outside them, a fraction or an empty field is put back as it was, without a message.
- The moves held are those of one rectangle 100 × 50 moved alone, 1 244 bytes each, rounded down to tens, `about 80,380 moves`, its digits grouped by commas as English writes them; a change of more shapes holds fewer.
- A memory or a number of steps applied bounds the history at once, as a step does; while a shape is being moved, a new text typed or a value refused, it waits for the next step.
- The browser keeps both values for the next session, as the checklists keep their ticks; a value it kept that the fields would refuse is put back to its default.
- The fewest old steps to drop are found by trying 1, 2, 4… then halving, so that a history within its memory is measured once and one a step over it twice; each measure still reads every shape the past and the steps to redo hold alone.
- Reset and each step of the guided test empty the history.

## The ideas behind it

**States rather than commands.** Figma, Inkscape and Penpot record, for each action, what it changed, so that it can be put back; Redux and the Memento keep the document's states instead. A command must be written for each of the twenty places the page changes the drawing, and one forgotten leaves a change that cannot be undone; a state restored is exactly the drawing it was. Since the editor's data are never modified, only replaced, a state shares every shape it did not change, as Immer and Clojure share what a change did not reach.

**Measured from the data.** GIMP bounds its history by a memory size it adds up from the parts of what it keeps, and always keeps a few steps whatever their size; the editor does the same, counting a text by its characters, as JavaScript stores a string.

| Term | In plain words |
| --- | --- |
| step | one finished change of the drawing, that Undo takes back and Redo does again |
| applied | a value taken by the page for good: Enter pressed, or the field left |
| state | the drawing and its shapes selected, as they were after a step |
| shared | the same shape kept once, by several states, instead of a copy in each |
