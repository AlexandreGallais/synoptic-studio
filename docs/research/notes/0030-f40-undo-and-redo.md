# 0030 — Sources of F40: undo and redo, by commands or by states, grouped, within a memory size

Spike SP-029, 2026-10-23, for F40's stories. Every page cited was read through the local library the same day; the code F40 touches was read at F39's head (`6d5ce4fb`, F40's branch opened on it).

## 1. What F40 needs

| Story | What it needs | Where it comes from |
| --- | --- | --- |
| EN-146, the history in the model | how a history is built, commands or states; how it shares what did not change; how its memory is measured and bounded | §2, §4, §5, ADR-0095 |
| US-074, undo and redo the drawing's actions | the keys and the buttons; a step at each finished action on the drawing; a key held; what undo restores besides the shapes | §3, §6 |
| US-075, undo and redo the panel's changes | a value typed, a choice, a button, a text, a color, as one step each | §3, §6 |
| US-076, the history's memory size | where the designer sets it, its default, what is dropped at the limit | §3, §5 |

The rules F40 starts from (Q7, `interaction.md` §7, 2026-10-22): one step per finished action — a drag at its release, a value typed when Enter is pressed or the field left, a text when its editing is left, an addition, a deletion; selecting alone never a step; the history bounded by a memory size the user sets, never by a count of steps. The keys (SP-027, `interaction.md` §1): Ctrl + Z undoes; Ctrl + Shift + Z and Ctrl + Y redo; Cmd for Ctrl on macOS; a key pressed while a field of the panel has the focus is the field's.

## 2. Commands and states

**Commands.** An action is an object that does and undoes itself. Myers and Kosbie's Amulet, a peer-reviewed account, traces command objects for undo to MacApp, InterViews and Gina; a command keeps its `VALUE` and its `OLD_VALUE`, and the default undo of a property changed "simply swaps the values"; the history is "a linear list", redo available only after an undo, a new command destroying the commands undone (`REF-MYERS-KOSBIE-1996`). A command can instead perform "the inverse operation", which "may turn out to be hard or even impossible to implement" (`REF-REFACTORING-GURU-COMMAND`, level 6, pointing to `REF-GOF`, which is not readable online).

**States.** The history keeps the document's states. Redux keeps `{ past, present, future }`: undo moves the last past state to the present and the present to the front of the future, redo the other way, any other action pushes the present onto the past and clears the future; it notes that a mutable model "usually forces you to rewrite parts of your application to use a specific data mutation pattern like Command", while an immutable one makes undo generic (`REF-REDUX-UNDO`). The Memento is a snapshot of a state kept by a caretaker, which "might consume lots of RAM" (`REF-REFACTORING-GURU-MEMENTO`, level 6).

**Between the two, diffs.** Immer records, while a state is produced, the patches of the change and their inverse, "as basis for undo/redo" (`REF-IMMER-PATCHES`): two states give the command that goes from one to the other.

**A rounding the inverse does not undo.** A sum in ECMAScript is the exact sum rounded to binary64 (`REF-ECMA-262-NUMBER-ADD`): a shape at x −1.8 × 10⁻¹⁵, the error a rounded rectangle's straight box can hold (`interaction.md` §2, How a drag lands), nudged by +1 and undone by −1 is at −1.7763568394002505 × 10⁻¹⁵ (computed in Node.js 24 on 2026-10-23). A command keeping the old value, or a state, gives back −1.8 × 10⁻¹⁵ exactly.

## 3. How editors build and group their history

"—" is what the pages read do not state.

| Behavior | Figma | Inkscape | Penpot | tldraw | GIMP | ProseMirror, Quill |
| --- | --- | --- | --- | --- | --- | --- |
| what a step holds | the property values replaced, a deleted object's properties in the client's undo buffer | the XML tree's changes of the action, as events | the changes to undo and to redo, the selection before and after | diffs of the store's records | the image's changes, a filter's layer before and after | the document's steps (ProseMirror inverts them, selectively) |
| a drag | — | one step, the XML written at the release | one entry by a transaction | one entry, the changes gathered until a mark | — | — |
| a key repeated | — | nudges along one axis merged by a key, expiring after 10 s | the repeats of one direction one change, ended 1 s after the last or 250 ms after the key's release | `squashToMark` makes several nudges one step | some operations "compressed", several in a row one point | typing grouped while changes come within 500 ms (ProseMirror) or 1 000 ms (Quill) |
| a text typed | — | — | "Editors handle their own undo's" while it is edited | — | the text tool undone whole: "you cannot undo individual letters" | per group of changes |
| the selection | — | not in the steps | restored by undo and redo, never a step of its own | recorded, without clearing redo | — | — |
| the bound | — | 200 steps, an option | 50 steps | — | a memory per image, its oldest steps dropped, 5 steps kept whatever their size; an eighth of the memory by default | 100 events |
| after an undo, a new step | — | clears the redo | drops the entries after the current one | clears the redo | the steps undone lost | — |

Sources: `REF-FIGMA-MULTIPLAYER`, `REF-INKSCAPE-DOCUMENT-UNDO`, `REF-INKSCAPE-SELECTION-MOVE`, `REF-PENPOT-UNDO`, `REF-PENPOT-TRANSFORMS` (Inkscape's and Penpot's code, level 5, read only, never copied), `REF-TLDRAW-HISTORY`, `REF-GIMP-UNDO`, `REF-GIMP-PREFS-RESOURCES`, `REF-GIMP-UNDO-CODE` (read only), `REF-PROSEMIRROR-HISTORY`, `REF-QUILL-HISTORY`; `REF-MYERS-KOSBIE-1996` on the selection.

Not read: Figma's help on undo, its grouping of nudges and of a text typed, and its redo key, none found in its help pages read; Sketch's and Affinity Designer's histories; Illustrator, Adobe's help answering 403 to the library. None is cited.

What the sources settle, two of them agreeing (`docs/research/README.md` §2):

- **A drag is one step**, at its release: Inkscape writes the drag at the release, Penpot gathers it in a transaction, tldraw until a mark. Already the rule (Q7).
- **The steps undone are dropped by a new step**: Amulet, Inkscape, Penpot, tldraw, GIMP, Redux. Taken as a rule for EN-146, the agent's reading.
- **The oldest steps go first at the bound**: Inkscape, Penpot, GIMP, ProseMirror. Taken as a rule for EN-146.
- **The selection is not a step**: Amulet ("not normally queued"), Inkscape, Penpot (restored with a step, never one alone); tldraw records it without clearing the redo. Already the rule (Q7).
- **Several keys repeated make one step**: Inkscape, Penpot, tldraw, GIMP, each its own way, which question 3 asks about.

What they leave open: whether undo restores the selection (question 2), how a key held is grouped (question 3), the color picker (question 4), a text placed and typed (question 5), the size and its bound (questions 6 and 7), what is shown (question 8), Ctrl + Z in a field (question 9).

## 4. Persistent data and structural sharing

An ordinary structure is "ephemeral", a change destroying its old version; "a persistent structure allows access to any version, old or new, at any time" (`REF-DRISCOLL-1989`, its abstract read). Path copying copies the node changed and its ancestors, the rest shared (`REF-MIT-6854-PERSISTENT`, level 6). Three libraries of different origins say the same of plain data: "Unchanged parts of a data tree don't need to be copied and are shared in memory with older versions of the same state" (`REF-IMMER`), "New versions reuse the parts that didn't change" (`REF-IMMUTABLE-JS`), and Clojure's collections create "'modified' versions, by utilizing structural sharing" (`REF-CLOJURE-DATA-STRUCTURES`).

The project's model is such data (ADR-0014, ADR-0025): `drawingWithoutShapes`, `shapeMovedBy` and the panel's functions return a new drawing whose untouched shapes are the objects passed in. A state of the drawing is then a list of references, one per shape, and the objects its action made: a drawing is a list, flat until E13, so the path copied is the list itself.

The playground breaks the sharing: it keeps the drawing as JSON text in the canvas's data attributes and reads it back with `JSON.parse` at each use (`readStoredDrawing`), new objects each time. A history sharing what did not change replaces each shape of a new state equal to one of the present state by that state's object (`sharedDrawing`, ADR-0095, decision 2), equal by their JSON text, as `KEPT_TEXT_METRICS` already keys a text by its JSON.

## 5. Measuring a history's memory in a browser

- **The browser's measures** are of the whole page: `performance.measureUserAgentSpecificMemory()` is used "to estimate the memory usage of a web application including all its iframes and workers", asynchronously, only in a secure and cross-origin isolated page, its bytes "not comparable across browsers", and it is not available in every browser (`REF-MDN-MEASURE-MEMORY`, `REF-WICG-MEASURE-MEMORY`); `performance.memory` is the JavaScript heap of Chromium alone, deprecated (`REF-MDN-PERFORMANCE-MEMORY`). Neither tells what the history holds.
- **An engine's bytes** are not the data's either: V8 holds every value as a tagged pointer, 32 bits with pointer compression, 64 without, a small integer inside it, a double boxed in an object of its own (`REF-V8-POINTER-COMPRESSION`).
- **A measure from the data**: GIMP bounds its history by a size it sums from its objects' parts, a name by its string (`REF-GIMP-UNDO-CODE`). ECMAScript defines what plain data holds: a string is a sequence of 16-bit code units (`REF-ECMA-262-STRING-TYPE`), a number a binary64, 8 bytes (`REF-ECMA-262-NUMBER-TYPE`).

ADR-0095, decision 4, counts the history so: for each shape a past or future state holds and the present does not, once, its JSON text's length times 2 bytes; plus 8 bytes for each reference to a shape and each index of a selection those states hold. A rectangle 100 × 50 at (10, 20) of radius 0, as the playground keeps it, `{"kind":"rectangle","shape":{…}}` with its four corners and their handles, is 614 characters, 1 228 bytes. A move of that rectangle alone in a drawing of one shape adds 1 236 bytes to the history, in a drawing of 100 rectangles 2 028 bytes (the old rectangle and 100 references), the selection left out, 8 bytes more with it: a size of 10 MB (10 000 000 bytes) holds about 8 090 such moves, or 4 930.

The count is in the core, ECMAScript alone (`JSON.stringify`, ADR-0025), a `domain` function tested by hand-computed sizes; no derivation is needed, the measure being a sum the ADR defines.

## 6. The code F40 touches

Read: `playground/kept-shapes/`, `playground/selection/`, `playground/selection-keys/`, `playground/selection-drag/`, `playground/placing/`, `playground/shape-drawing/`, `playground/mount/`, `playground/redraw/`, `playground/drawing/`, `playground/toolbar/`, the listeners of `playground/position/`, `playground/shape/`, `playground/points/`, `playground/handles/`, `playground/text/`, `playground/text-placement/`, `playground/color-fields/`, `playground/picker-controls/`, `playground/fonts/`, `playground/installed-fonts/`; `src/model/drawing/`, `src/model/moving/`.

**Every place that changes the drawing**, and where its step would be taken:

| Action | Where | Heard on | The step taken |
| --- | --- | --- | --- |
| a shape placed by a click | `placing/listen-to-placing.ts` | `click` on the canvas, a tool armed | at the click |
| a shape drawn by a drag | `shape-drawing/listen-to-shape-drawing.ts` | `pointerup`; `pointercancel` draws nothing | at the release |
| shapes moved by a drag | `selection-drag/listen-to-selection-drag.ts` | `pointerup`; `pointercancel` gives the drawing back | at the release; none when cancelled |
| shapes nudged by an arrow | `selection-keys/listen-to-nudge-keys.ts` | `keydown`, each repeat | question 3 |
| shapes, or points, deleted | `selection-keys/listen-to-selection-keys.ts`; `points/listen-to-point-fields.ts` | `keydown` Delete, Backspace; the Delete button | at the key or the click |
| one shape's values: width, height, corners, radius, X, Y, angle, a point's radius | `mount/listen-to-inputs.ts`, `position/`, `shape/listen-to-radius.ts` | `input`, the drawing redrawn at each | when the field is left or Enter pressed (Q7) |
| one shape's choices: its kind, the anchor, Stretched, a handle's mode or method, a point's x or y | `mount/listen-to-inputs.ts`, `shape/listen-to-anchor.ts`, `handles/`, `points/` | `change` | at the change |
| one shape's buttons: turn, convert to a free shape, add a midpoint, handles on or off | `position/listen-to-turn.ts`, `points/listen-to-point-fields.ts`, `handles/` | `click` | at the click |
| several shapes' X, Y, W, H | `selection/listen-to-several-fields.ts` | `change` | at the change |
| a text's content and options, its box fitted or reset, bold, italic, weight, its place | `text/listen-to-text-fields.ts`, `text-placement/` | `input`, `change`, `click` | the content when its field is left (Q7); a choice or a button at once |
| a text's colors | `color-fields/`, `picker-controls/` | a field's `change`, a drag on the plane, a slider's keys | question 4 |
| a text's family, a font imported | `installed-fonts/`, `fonts/` | `change` | at the change |
| Reset, a step of the guided test | `toolbar/listen-to-toolbar.ts`, `mount/mount-playground.ts` | `click` | the history emptied, the page as it opens (agent's choice, US-074) |

Not a step: a click, Shift + click, a selection rectangle, Ctrl + A, Escape (selecting); Shape or Points mode, a tool armed, the hover, a point selected in the list or on the drawing, the window resized, the export.

What the code shows:

- **The drawing is kept in four places**, `DRAWING_KEYS`: the shapes below the one the panel edits, that one as last drawn, those above it, and the indices of several selected; the shape edited is the panel's fields, written back as last drawn at each redraw (`showShape`), none while it is refused (`showRefusedShape`). `readKeptDrawing` gives the whole drawing in the order drawn, `readSelectedShapes` the selection.
- **A state is restored in two calls**: the drawing written below (`writeStoredDrawing`), the other keys emptied, then `writeShapeSelection`, which splits the drawing around one shape selected and loads it into the panel (`loadPickedShape`), or keeps several by their indices, as Delete does (`removeSelectedShapes`).
- **A redraw waits for the next frame** (`scheduleRedraw`, US-048): the drawing is read for a step once the page is redrawn from the action, not at the event itself.
- **The panel redraws on `input`**, each character typed: a step is taken when the field is left or Enter pressed, as Q7 asks, or when a choice is made (`change`, `REF-MDN-CHANGE-EVENT`); a value typed back as it was makes no step, the drawing being unchanged.
- **A shape refused** in the panel is not in the drawing (`showRefusedShape` empties the shape edited): no step is taken while it is refused (agent's choice, US-075), so that a value out of range typed and corrected is one step, and a refused shape dropped by a click elsewhere, as today, is a step that undo gives back with its last valid values.
- **The keys are heard in one place**, `listenToSelectionKeys`, outside the fields (`readKeyField`) and not during a press with the Selector (`readPressedMove`); Ctrl + Z, Ctrl + Shift + Z and Ctrl + Y go beside Ctrl + A there.
- **Folder sizes** (15 modules a signal, `docs/conventions/files.md`): `playground/selection/` holds 14, `playground/selection-keys/` 4, `playground/selection-drag/` 12; the history's playground goes in a folder of its own, `playground/history/`, its model in `src/model/history/`.

## 7. The stories and their size

| Story | Tasks | What it holds |
| --- | --- | --- |
| EN-146, the history in the model | 2 | `src/model/history/`: the history's type, a step, undo, redo, the shapes shared, the size, the oldest dropped; `domain` functions, hand-computed sizes |
| US-074, undo and redo the drawing's actions | 3 | `playground/history/`: the history of the page, a step at a click placing, a drag's release, a deletion, the arrows; Ctrl + Z, Ctrl + Shift + Z, Ctrl + Y, two buttons in the toolbar; a state restored; Reset |
| US-075, undo and redo the panel's changes | 2 | a step when a field is left or Enter pressed, at a choice, at a button, for one shape, several, the points, the handles, a text, its colors and fonts; a shape refused |
| US-076, the history's memory size | 2 | the size the designer sets, its default, the size used shown, the oldest steps dropped at the limit |

F40 holds four stories and its audit; nothing goes to an epic of its own. A list of the steps to go back to, as Inkscape's Undo History, Penpot's history panel and GIMP's dialog, would be a fifth story (question 8): proposed for E20, as a feature of its own, if the Product Owner wants it.

## 8. What remains open

The questions for the Product Owner are listed in full in SP-029, each with options and an example: the way the history is built (1, ADR-0095), the selection after an undo (2), a key held (3), the color picker (4), a text placed and typed (5), the size's default and place (6), the limit (7), what is shown (8), Ctrl + Z in a field (9).

Left out of F40, not asked now: undoing in a drawing shared with others (Figma's and ProseMirror's histories, E11); a history kept after the page is closed, which no source read keeps (GIMP's "is not saved"); undoing a step in the middle while keeping the later ones (ProseMirror's selective history, Amulet's).
