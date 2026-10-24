import { isBrowserKey } from "../../src";
import { readListenerSignal } from "../listeners";
import { readKeyField } from "../selection-keys";
import { POINTER_OFFSETS } from "../view";

import { applyZoom } from "./apply-zoom";

import type { ZoomName } from "./zoom-name";

/** The keys that zoom by a step, by the character they send: `+`, or `=`, Inkscape's, and `-`. */
const STEP_KEYS: ReadonlyMap<string, ZoomName> = new Map([
  ["+", "in"],
  ["-", "out"],
  ["=", "in"],
]);

/** The keys pressed with `Shift`, by their place on the keyboard: `0`, `1` and `2` of the top row. */
const SHIFT_CODES: ReadonlyMap<string, ZoomName> = new Map([
  ["Digit0", "actual"],
  ["Digit1", "drawing"],
  ["Digit2", "selection"],
]);

/**
 * Reads the zoom of a key: `Shift` with a digit of the top row by its place, the character it
 * sends changing with the layout (`)` on a US keyboard, `0` on a French one), else a step by its
 * character.
 *
 * @kind procedure
 * @param event - the key pressed
 * @returns the zoom; undefined for a key of no zoom
 * @see docs/domain/interaction.md#_1-tools
 */
function readKeyZoom(event: KeyboardEvent): ZoomName | undefined {
  return (event.shiftKey ? SHIFT_CODES.get(event.code) : undefined) ?? STEP_KEYS.get(event.key);
}

/**
 * Reads the zoom a key pressed asks (`readKeyZoom`): none with `Ctrl`, `Cmd` or `Alt` held, so
 * that the browser keeps its own zoom (`isBrowserKey`), for a key already taken by a listener before this one, its default prevented, or typed in
 * a field of the panel (`readKeyField`).
 *
 * @kind procedure
 * @param event - the key pressed
 * @returns the zoom; undefined for a key of no zoom
 * @see docs/domain/interaction.md#_1-tools
 */
function readZoomKey(event: KeyboardEvent): ZoomName | undefined {
  const isTaken = [
    isBrowserKey(event),
    event.defaultPrevented,
    readKeyField(event) !== undefined,
  ].includes(true);

  return isTaken ? undefined : readKeyZoom(event);
}

/**
 * Listens to the keys of the zoom outside the panel's fields (SP-030's answer 3, Figma's and
 * Penpot's): `+`, or `=`, Inkscape's, zooms in by a step and `-` out, about the pointer when it is
 * over the drawing (`POINTER_OFFSETS`), a pan drag's included, else about the canvas's centre; `Shift` + `0` gives 100 %
 * about the canvas's centre, `Shift` + `1` fits the drawing and `Shift` + `2` the selection
 * (`applyZoom`), the digit read by its key's place, whatever the layout sends with `Shift`; their
 * default prevented, then a redraw asked. A key typed in a field is the field's (`readKeyField`,
 * `interaction.md` §1); with `Ctrl`, `Cmd` or `Alt` held, or taken by a listener before this one,
 * it is left to the browser (`readZoomKey`).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page
 * @see docs/domain/interaction.md#_1-tools
 */
export function listenToZoomKeys(document: Document, update: () => void): void {
  document.addEventListener(
    "keydown",
    (event) => {
      const name = readZoomKey(event);

      if (name === undefined) {
        return;
      }

      event.preventDefault();
      applyZoom(document, name, POINTER_OFFSETS.get(document));
      update();
    },
    { signal: readListenerSignal(document) },
  );
}
