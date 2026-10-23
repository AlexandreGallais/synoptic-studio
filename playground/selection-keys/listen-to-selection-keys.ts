import { readOpenSwatch } from "../color-picker";
import { readKeptDrawing } from "../kept-shapes";
import { readMode } from "../page";
import { writeDeletedPoints } from "../points";
import { removeSelectedShapes, writeShapeSelection } from "../selection";

import { readKeyField } from "./read-key-field";
import { readPressedMove } from "./read-pressed-move";

/** What a key of the selection does to the page. */
type SelectionKey = (document: Document, event: KeyboardEvent) => void;

/**
 * Selects every shape, in the order drawn (F37), its default, the page's text selected, prevented;
 * in Points mode, which edits one shape, the key is left to the browser, as before.
 *
 * @kind procedure
 * @param document - page document
 * @param event - Ctrl + A, or Cmd + A, pressed
 * @see docs/domain/interaction.md#_2-selector
 */
function selectEveryShape(document: Document, event: KeyboardEvent): void {
  if (readMode(document) === "points") {
    return;
  }

  event.preventDefault();
  writeShapeSelection(document, readKeptDrawing(document).keys().toArray());
}

/**
 * Selects no shape (F37), unless a color picker is open, which Escape closes alone (F16).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_2-selector
 */
function selectNoShape(document: Document): void {
  if (readOpenSwatch(document) === undefined) {
    writeShapeSelection(document, []);
  }
}

/**
 * Removes what is selected (F37): in Points mode, which edits one shape, its points selected, as
 * before (F08, `writeDeletedPoints`), never the shape; otherwise the shapes selected
 * (`removeSelectedShapes`).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_2-selector
 */
function removeSelection(document: Document): void {
  if (readMode(document) === "points") {
    writeDeletedPoints(document);
  } else {
    removeSelectedShapes(document);
  }
}

/** The keys of the selection pressed with Ctrl, or Cmd: A selects every shape. */
const COMMAND_KEYS: ReadonlyMap<string, SelectionKey> = new Map([
  ["A", selectEveryShape],
  ["a", selectEveryShape],
]);

/**
 * The keys of the selection pressed alone: Escape selects none; Delete, and Backspace, sent by
 * the key labelled "delete" on a Mac, remove what is selected.
 */
const PLAIN_KEYS: ReadonlyMap<string, SelectionKey> = new Map([
  ["Backspace", removeSelection],
  ["Delete", removeSelection],
  ["Escape", selectNoShape],
]);

/**
 * Reads what a key pressed does to the selection: Ctrl + A, or Cmd + A, selects every shape,
 * Escape none, Delete or Backspace removes what is selected.
 *
 * @kind procedure
 * @param event - the key pressed
 * @returns what it does; undefined for a key of no selection
 * @see docs/domain/interaction.md#_1-tools
 */
function readSelectionKey(event: KeyboardEvent): SelectionKey | undefined {
  const isCommand = event.ctrlKey || event.metaKey;

  return isCommand ? COMMAND_KEYS.get(event.key) : PLAIN_KEYS.get(event.key);
}

/**
 * Applies a key of the selection pressed outside the panel's fields, then redraws; while a press
 * with the Selector moves shapes, none (`readPressedMove`, F39), so that the drag, landing the
 * drawing it kept at the press, never undoes a deletion or a selection made meanwhile.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the key pressed
 * @param update - redraws the page from the values typed
 * @see docs/domain/interaction.md#_2-selector
 */
function applySelectionKey(document: Document, event: KeyboardEvent, update: () => void): void {
  const select = readSelectionKey(event);

  const isTaken = [
    readKeyField(event) !== undefined,
    readPressedMove(document) !== undefined,
  ].includes(true);

  if (select === undefined || isTaken) {
    return;
  }

  select(document, event);
  update();
}

/**
 * Listens to the keys of the selection (F37), outside the panel's fields, which keep every key
 * they are typed in, the text's Content and the color picker's sliders included, and none while a
 * press with the Selector moves shapes (F39): Ctrl + A, or Cmd + A, selects every shape, in
 * Shape mode; Escape selects none, unless a color picker is open, which Escape closes alone (F16);
 * Delete or Backspace removes the shapes selected, or in Points mode the points selected (F08);
 * then redraws. Heard as the key goes down to its target, before the picker closes on it.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see docs/domain/interaction.md#_1-tools
 */
export function listenToSelectionKeys(document: Document, update: () => void): void {
  document.addEventListener(
    "keydown",
    (event) => {
      applySelectionKey(document, event, update);
    },
    { capture: true },
  );
}
