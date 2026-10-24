# ADR-0095 — Undo and redo by states of the drawing sharing what did not change

**Status**: Accepted (2026-10-23): the Product Owner left the choice to the agent, asking for the way that makes most sense for the project rather than the fastest (SP-029's answer 1); research note 0030.

**Note (2026-10-23, AUD-042)**: decision 4's "a 64-bit slot" is the slot of a tagged value without pointer compression, an upper bound: with compression V8 holds it in 32 bits (`REF-V8-POINTER-COMPRESSION`); the 8 bytes counted are unchanged. Each shape's text length is kept once measured (ADR-0096).

## Context

- The rules of undo and redo were set on 2026-10-22 (Q7, `interaction.md` §7): one step per finished action — a drag at its release, a value typed when Enter is pressed or the field left, a text when its editing is left, an addition, a deletion; selecting alone is never a step; the history is bounded by a memory size the user sets, never by a count of steps. How it is built, commands or states, was left to F40's spike, from read sources.
- The model is plain data that no function modifies (ADR-0014, ADR-0025): a change returns a new drawing, the shapes it did not touch passed on as they are.
- The playground keeps the drawing as JSON text in the canvas's data attributes, split around the shape the panel edits (`DRAWING_KEYS`), that shape read from the panel's fields and written back as last drawn at each redraw, several shapes selected kept by their indices; `writeShapeSelection` writes a whole drawing and a selection back and loads one shape into the panel. About twenty places change the drawing (note 0030, §5): placing, drawing, moving, nudging, deleting, and the panel's fields and buttons for one shape, several, the points, the handles, a text, its colors and fonts, most of them redrawing on every `input`.
- The sources describe two ways (note 0030, §2 to §4):
  - **Commands**: each action is an object that does and undoes itself, either by keeping the values it replaced (`REF-MYERS-KOSBIE-1996`: `VALUE`, `OLD_VALUE`, "simply swaps the values") or by an inverse operation, which "may turn out to be hard or even impossible to implement" (`REF-REFACTORING-GURU-COMMAND`). Figma keeps the properties an action replaced, a deleted object's in "the undo buffer" (`REF-FIGMA-MULTIPLAYER`); Inkscape logs every change of its XML tree (`REF-INKSCAPE-DOCUMENT-UNDO`); Penpot keeps each entry's `undo-changes` and `redo-changes` (`REF-PENPOT-UNDO`); tldraw diffs of its records (`REF-TLDRAW-HISTORY`). All four hold objects with ids, edited by several people at once in Figma and Penpot.
  - **States**: the history keeps the document's states, `{ past, present, future }`, undo going back to the state before (`REF-REDUX-UNDO`; the Memento, `REF-REFACTORING-GURU-MEMENTO`), which costs "lots of RAM" when each state is a copy. With immutable data a new state "reuses the parts that didn't change" (`REF-IMMUTABLE-JS`), "shared in memory with older versions of the same state" (`REF-IMMER`; `REF-CLOJURE-DATA-STRUCTURES`, structural sharing; `REF-DRISCOLL-1989`, persistence), so that a state costs what it changed and a reference per part.
- No browser measures the memory of a part of a page: `measureUserAgentSpecificMemory()` estimates the whole page's, asynchronously, in a cross-origin isolated page, in bytes "not comparable across browsers" (`REF-MDN-MEASURE-MEMORY`, `REF-WICG-MEASURE-MEMORY`), and `performance.memory` is Chromium's heap, deprecated (`REF-MDN-PERFORMANCE-MEMORY`). GIMP bounds its history by a size it sums from its objects' parts (`REF-GIMP-UNDO-CODE`).

## Decision

1. **A history of states.** The history is the drawing's past states, its present state and its future states, as Redux's (`REF-REDUX-UNDO`): a step pushes the present onto the past and clears the future; undo takes the last past state back, the present going to the front of the future; redo the other way. A state is the drawing in the order drawn, each shape with the kind the panel showed it as (`KeptShape`), and the selection to restore if SP-029's answer 2 keeps one.
2. **Shared.** A new state keeps, by reference, each shape equal to one of the present state's, equal by their JSON text, so that a state holds new objects only for the shapes its action changed (`sharedDrawing`), as path copying keeps what a change did not reach (`REF-MIT-6854-PERSISTENT`).
3. **A step where an action finishes.** At the end of each action of note 0030, §5 — a release, a click placing a shape, a key, a field left or Enter pressed, a choice, a button —, once the page is redrawn from it, the drawing is compared with the present state: the same, no step is taken, so that selecting alone, or a value typed back as it was, never makes one.
4. **Measured from its data.** The history's size is, for each shape held by a past or future state and not by the present, counted once, the length of its JSON text times 2 bytes, a string being a sequence of 16-bit code units (`REF-ECMA-262-STRING-TYPE`), plus 8 bytes for each reference to a shape and each index of a selection the past and future states hold, a 64-bit slot (`REF-V8-POINTER-COMPRESSION`). It is computed in the core with ECMAScript alone, `JSON` being part of it (ADR-0025), and it is a count of the data, not the engine's bytes, which no browser gives for a part of a page.
5. **Bounded by the size set.** After each step, the oldest past states are dropped while the size exceeds the size the designer sets, as GIMP, Inkscape and Penpot drop the oldest (`REF-GIMP-PREFS-RESOURCES`, `REF-INKSCAPE-DOCUMENT-UNDO`, `REF-PENPOT-UNDO`); what is kept below it, and a step larger than the whole size, follow SP-029's answer 6.
6. **Where.** The history and its functions are plain data and `domain` functions of `src/model/history/`; the playground keeps one history per page in a module's constant, as `KEPT_TEXT_METRICS`, records the steps, and restores a state by writing its drawing and its selection as `writeShapeSelection` writes them.

Example (A, a rectangle 100 × 50 at (10, 20) of radius 0, 614 characters of JSON as the playground keeps it; B the same at (200, 20), 615): A and B drawn, A nudged to (11, 20). The present holds A′ and B; the past state holds A and B, B shared with the present by reference. The history holds A, 1 228 bytes, and two references, 16 bytes: 1 244 bytes, 1 252 with A's index kept as its selection. Kept as a copy, the past state would hold 2 474 bytes.

## Reasons

| Option | For | Against |
| --- | --- | --- |
| States sharing what did not change (chosen) | one way of recording for every action, at a few places where actions finish, none needing an undo of its own; selecting never a step, by the comparison; a state restored is exactly the drawing it was, its fractions included; the data already immutable, sharing it costs nothing more (`REF-REDUX-UNDO`, `REF-IMMER`) | a reference per shape and per step, 8 bytes, 800 bytes a step in a drawing of 100 shapes; the drawing compared at each finished action, one JSON text per shape; the playground's drawing, read from JSON at each use, needs `sharedDrawing` to share |
| Commands keeping the values each action replaced (Figma, Penpot, Inkscape, tldraw, `REF-MYERS-KOSBIE-1996`) | only what changed is kept; what each step did can be named, for a list of the steps | a recording for each of the twenty places that change the drawing, each panel field, point and handle among them, a place forgotten leaving a change undone or a history wrong; GIMP warns that an action recording its undo wrongly can corrupt the history (`REF-GIMP-UNDO`) |
| Commands by inverse operations (`REF-REFACTORING-GURU-COMMAND`) | the least memory | an inverse for each action, "hard or even impossible" for some (a deletion, a conversion to a free shape); in binary64 a sum rounds (`REF-ECMA-262-NUMBER-ADD`), so a shape at x −1.8 × 10⁻¹⁵ nudged by +1 and undone by −1 is at x −1.7763568394002505 × 10⁻¹⁵ |
| States kept as copies (the Memento alone) | the simplest | each step the whole drawing: 1 228 bytes a rectangle, 122 800 bytes a step in a drawing of 100, against 2 028 shared |

## Consequences

- F40's stories (SP-029): the history's model and its measure; the keys and the buttons, the canvas's actions recorded; the panel's changes recorded; the size the designer sets.
- A state is compared and shared by the JSON text of each shape: a drawing of 1 000 shapes is 1 000 texts of about 600 characters at each finished action.
- The future state of a step undone is dropped by the next step, as in every source read; a branching history (ProseMirror's selective undo, `REF-PROSEMIRROR-HISTORY`) is left out.
- Sharing a drawing with others (E11) would need what a step changed, which two states give by comparison (`REF-IMMER-PATCHES`), without changing this decision.
- `interaction.md` §7 and Q7 name the way chosen once the Product Owner decides.

## References

ADR-0005, ADR-0014, ADR-0025, research note 0030, SP-029, `REF-REDUX-UNDO`, `REF-IMMER`, `REF-IMMUTABLE-JS`, `REF-CLOJURE-DATA-STRUCTURES`, `REF-DRISCOLL-1989`, `REF-MYERS-KOSBIE-1996`, `REF-FIGMA-MULTIPLAYER`, `REF-INKSCAPE-DOCUMENT-UNDO`, `REF-PENPOT-UNDO`, `REF-TLDRAW-HISTORY`, `REF-GIMP-UNDO`, `REF-GIMP-PREFS-RESOURCES`, `REF-GIMP-UNDO-CODE`, `REF-ECMA-262-STRING-TYPE`, `REF-V8-POINTER-COMPRESSION`, `REF-MDN-MEASURE-MEMORY`
