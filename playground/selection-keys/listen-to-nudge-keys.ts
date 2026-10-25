import { heldNudgeShift, isBrowserKey, landedNudgeShift, nudgeShift } from "../../src";
import { readSelectedShapes } from "../kept-shapes";
import { readListenerSignal } from "../listeners";
import { readMode } from "../page";
import { readMovedDrawing, readSelectionMove, writeMovedDrawing } from "../selection-drag";

import { listenToHeldArrows } from "./listen-to-held-arrows";
import { readHeldArrows } from "./read-held-arrows";
import { readKeyField } from "./read-key-field";
import { readPressedMove } from "./read-pressed-move";

import type { Point } from "../../src";

/**
 * Reads how far a key pressed nudges the shapes selected: an arrow, or Shift + an arrow, by the
 * sum of the arrows held (`heldNudgeShift`, `readHeldArrows`), in Shape mode; none for a
 * key other than an arrow (`nudgeShift`), nor when the key is not the drawing's: Ctrl, Cmd or Alt
 * held, so that the browser keeps the arrows it gives them (`isBrowserKey`), the key already taken by a listener before
 * this one, its default prevented, or the focus in a field of the panel, the color picker's
 * sliders included, which keeps every key (`readKeyField`).
 *
 * @kind procedure
 * @param document - page document
 * @param event - the key pressed
 * @returns the shift; undefined for a key that nudges nothing
 */
function readNudge(document: Document, event: KeyboardEvent): Point | undefined {
  const isTaken = [
    isBrowserKey(event),
    event.defaultPrevented,
    readKeyField(event) !== undefined,
    readMode(document) === "points",
    nudgeShift(event.key, false) === undefined,
  ].includes(true);

  return isTaken ? undefined : heldNudgeShift(readHeldArrows(document), event.shiftKey);
}

/**
 * Moves the shapes selected by a nudge: their anchor, the top-left of their dashed box,
 * moved by it and landed on integers as at a drag's release (`landedNudgeShift`), every shape
 * selected going by that same shift, a text never measured left as it is (`readSelectionMove`,
 * `readMovedDrawing`), written back selected (`writeMovedDrawing`), then the page is redrawn;
 * while a press with the Selector moves shapes, none, the drag landing them at the release
 * (`readPressedMove`).
 *
 * @kind procedure
 * @param document - page document
 * @param shift - the nudge, before it lands
 * @param update - redraws the page from the values typed
 */
function moveSelectedShapes(document: Document, shift: Point, update: () => void): void {
  if (readPressedMove(document) !== undefined) {
    return;
  }

  const selected = readSelectedShapes(document);
  const move = readSelectionMove(document, selected, selected);

  const landed = landedNudgeShift(move.anchor, shift);

  writeMovedDrawing(document, readMovedDrawing(move, landed), selected);
  update();
}

/**
 * Moves the shapes selected by an arrow, the page then not scrolling
 * (`moveSelectedShapes`); with no shape selected, or a key that nudges nothing (`readNudge`), the
 * key is left to the browser.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the key pressed
 * @param update - redraws the page from the values typed
 */
function moveNudgedShapes(document: Document, event: KeyboardEvent, update: () => void): void {
  const shift = readNudge(document, event);

  if (shift === undefined || readSelectedShapes(document).length === 0) {
    return;
  }

  event.preventDefault();
  moveSelectedShapes(document, shift, update);
}

/**
 * Listens to the arrow keys: outside the panel's fields, the color picker's sliders
 * included, which keep them, an arrow nudges the shapes selected in Shape mode by 1, or 10 with
 * Shift (`nudgeShift`), landed on integers, at each key the browser sends, its repeats included, by the sum of the
 * arrows held (`listenToHeldArrows`), so that Up and Right held move diagonally, then
 * redraws; the page then does not scroll. With no shape selected, in Points mode, or with Ctrl, Cmd or Alt
 * held, the key is left to the browser; while a press with the Selector moves shapes, it moves
 * nothing.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 */
export function listenToNudgeKeys(document: Document, update: () => void): void {
  listenToHeldArrows(document);

  document.addEventListener(
    "keydown",
    (event) => {
      moveNudgedShapes(document, event, update);
    },
    { signal: readListenerSignal(document) },
  );
}
