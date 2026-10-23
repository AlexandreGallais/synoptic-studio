---
id: SP-029
epic: E02
feature: F40
title: Refine undo and redo
status: done
points: 3
---

# E02 · F40 · SP-029 — Refine undo and redo

The spike that opens F40, its research done in full before its stories (Product Owner, 2026-10-22): the sources, the code each story will touch, every question at once, the feature's size known before any code.

## Acceptance criteria

- Given the two ways a history is built — commands each undone by its inverse, and states of the document kept and restored (snapshots, persistent data sharing what did not change) — when the spike ends, then each is described from read sources (the Command and Memento patterns, Figma's, Inkscape's, Penpot's, a text editor's, immutable data structures), with what it costs in code and in memory for this project, whose model is plain data that no function modifies (ADR-0014, ADR-0025), and the way F40 takes is chosen with its reasons, in an ADR.
- Given the rules of 2026-10-22 (one step per finished action, selection never a step, a memory size set by the user) and the playground's code after F39 (every place that changes the drawing: placing, drawing, moving, nudging, deleting, typing in the panel, editing points, a text), when the spike ends, then how the memory used is measured, what happens at the limit, which actions group into one step (a held arrow key, a text typed), and where undo and redo are offered (keys of `interaction.md` §1, buttons) are written, and F40's stories, 2 to 4 of at most 3 tasks each, criteria with numeric examples; a larger scope proposed as an epic of its own, with an example.
- Given what the sources and the code leave open, when the spike ends, then every question for the Product Owner is listed at once, each with options and a numeric example.

## Questions

- Commands or states: which does a plain-data model, never modified in place, call for?
- How is a history's memory size measured in a browser, and what is dropped when it is reached?
- Which actions make one step: a key held, several values typed, a text's characters?

The sources, the code read and each story's size: [note 0030](../../research/notes/0030-f40-undo-and-redo.md). The way proposed: ADR-0095, a history of the drawing's states, each sharing by reference the shapes it did not change, a step taken where an action finishes when the drawing differs from the last state, its size counted from its data (a shape's JSON text at 2 bytes a character, 8 bytes a reference), the oldest states dropped beyond the size set. The stories: EN-146 (the history in the model), US-074 (undo and redo what is done on the drawing, by the keys and the toolbar), US-075 (what is typed and chosen in the panel), US-076 (the memory size); all four draft until the questions below are answered. Settled by the sources, two agreeing: a step after an undo drops the steps undone (Amulet, Inkscape, Penpot, tldraw, GIMP, Redux); the oldest steps go first at the bound (Inkscape, Penpot, GIMP, ProseMirror); a drag is one step at its release (Inkscape, Penpot, tldraw). F40 holds four stories and its audit: no epic of its own is needed, unless answer 8 (c) asks for a list of the steps.

## Questions for the Product Owner

The examples draw, unless said otherwise, a rectangle A, 100 × 50 at (10, 20) of radius 0, a rectangle B, the same at (200, 20), and C, the same at (400, 20), drawn in that order, the focus outside the panel's fields. Sizes are counted as ADR-0095 proposes: A as the playground keeps it is 614 characters of JSON, 1 228 bytes; a reference to a shape 8 bytes; 1 MB is 1 000 000 bytes.

1. **How the history is built** (ADR-0095, EN-146). A selected and nudged by Right to (11, 20).
   - (a) States sharing what did not change, as ADR-0095 proposes, as Redux keeps them with immutable data: one way of recording for every action, at the few places where an action finishes, the drawing compared with the last state; the history holds the old A and two references, 1 244 bytes; a shape at x −1.8 × 10⁻¹⁵ nudged and undone is back at −1.8 × 10⁻¹⁵ exactly.
   - (b) Commands keeping the values each action replaced, as Figma, Inkscape and Penpot: the history holds the old A, 1 228 bytes; each of the about twenty places that change the drawing (note 0030, §6) records what it replaced, the panel's fields, points, handles and text among them, likely a fifth story.
   - (c) Commands undone by an inverse operation: the least memory, a nudge kept as (1, 0), 16 bytes; a shape at x −1.8 × 10⁻¹⁵ nudged by Right and undone by the inverse is at −1.7763568394002505 × 10⁻¹⁵, a sum rounding in binary64; a deletion still keeps the shapes it removed.
2. **What is selected after an undo or a redo** (US-074). A selected and nudged by Right to (11, 20), then B clicked, then Ctrl + Z.
   - (a) The selection as it was when the step was taken, as Penpot restores the selection before an entry on undo and after it on redo: A back at (10, 20), A selected and loaded into the panel; Ctrl + Shift + Z, A at (11, 20), still selected; B and C deleted and undone, B and C back and selected.
   - (b) The selection kept as it is, the shapes still there: A back at (10, 20), B still selected; B and C deleted and undone, B and C back, none selected.
   - (c) No shape selected after an undo or a redo: A back at (10, 20), none selected.
3. **A key held, and nudges in a row** (US-074). A at (10, 20): Right held for about a second, 30 keys sent, A at (40, 20); released, then Right pressed once more, A at (41, 20); then Ctrl + Z twice.
   - (a) Each key sent is a step, the repeats included: (40, 20), then (39, 20); the 31 nudges are 31 steps.
   - (b) A key held, from its press to its release, is one step, as Penpot ends one 250 ms after the release: (40, 20), then (10, 20).
   - (c) Nudges in a row along one axis are one step until another action or 10 s without a nudge, as Inkscape merges them: (10, 20) at the first Ctrl + Z; a Down between them ends the group.
4. **The color picker** (US-075). A text's color `#ff8000`, the picker opened, the hue slider's Right pressed 5 times, 30° to 35°, the plane dragged once, the picker closed, then Ctrl + Z.
   - (a) Each change finished is a step: each key on a slider, a drag on the plane at its release, a hexadecimal on Enter or when its field is left: 6 steps, the first Ctrl + Z undoing the plane's drag.
   - (b) The picker, from its opening to its closing, is one step, as a value typed is one when its field is left: one Ctrl + Z gives `#ff8000` back.
   - (c) As (a), the keys in a row on one slider grouped as answer 3 groups the arrows: 2 steps with 3 (b) or 3 (c).
5. **A text placed and typed** (US-075). The Text tool clicks at (100, 50), placing `Text`, its Content field selected; `Valve` typed, the field left; then Ctrl + Z.
   - (a) Two steps, the text placed and its content: the first Ctrl + Z gives `Text`, the second removes the text.
   - (b) One step, the text placed with what is typed until its field is first left, as GIMP undoes a new text whole: one Ctrl + Z removes the text; a content typed later in a text already there is a step of its own.
6. **The size: its default, and where it is set** (US-076). A move of A alone adds 1 236 bytes to the history; in a drawing of 100 rectangles, 2 028 bytes.
   - Default: (a) 10 MB, about 8 090 moves of A alone, 4 930 in a drawing of 100; (b) 50 MB, about 40 450, or 24 650; (c) 100 MB, about 80 900, or 49 300.
   - Where: (i) a field of the playground beside the Undo and Redo buttons, kept in the browser for the next session, as the checklists keep their ticks; (ii) the same field, back to the default when the page is opened again, the standard place for settings (Keys editable, E20) taking it later; (iii) no field in F40, the default alone, the setting coming with E20's settings.
7. **At the limit** (EN-146, US-076). A size of 1 MB; 1 000 rectangles selected and `+50` typed in W: the step holds the 1 000 rectangles as they were and 1 000 references, 1 236 000 bytes, more than the whole size.
   - (a) The last step is always kept, alone above the size if need be, every older step dropped: one Ctrl + Z gives the 1 000 widths back, nothing before it.
   - (b) A minimum of steps is kept whatever their size, as GIMP keeps 5: the 4 steps before it kept too, the history at 1.236 MB and theirs.
   - (c) A step larger than the size is not kept: the history emptied, a line saying the change is too large to be undone.
8. **What is shown** (US-074, US-076).
   - (a) The Undo and Redo buttons in the toolbar, each disabled while there is nothing to undo or redo, and the size used beside the size set: after A's nudge, History 1.2 kB of 10 MB (1 244 bytes).
   - (b) The buttons alone.
   - (c) As (a), and a list of the steps to go back to by a click, as Inkscape's Undo History, Penpot's history panel and GIMP's dialog: a fifth story, so proposed as a feature of E20 rather than in F40, for example a list of the last steps each named by its action (Move, Delete, Width), a click on Move going back to before it.
9. **Ctrl + Z with the focus in a field** (US-074). 150 typed in A's W over 100 and Enter pressed, the focus staying in W, A 150 wide, one step; then Ctrl + Z.
   - (a) The field keeps the key, as every key (`interaction.md` §1, AUD-041): the browser undoes the field's text to 100, A drawn 100 wide at once, a step taken when the field is left; Ctrl + Z outside the field then gives 150 again.
   - (b) A field whose value is applied, nothing typed since, leaves Ctrl + Z, Ctrl + Shift + Z and Ctrl + Y to the drawing: A 100 wide, the field showing 100; once a character is typed, the field keeps them.
   - (c) The keys are always the drawing's: what is typed and not applied is dropped, then the last step undone.

## Tasks

One task = one commit, referenced as `SP-029.Tn`.

- [x] T1 — The sources found, read and recorded, the research note, the ADR proposed (2 h)
- [x] T2 — The code read, F40's stories written, the questions for the Product Owner listed (2 h)
