import { sizedRedoneHistory, sizedUndoneHistory } from "../../src";
import { readListenerSignal } from "../listeners";
import { readKeyField } from "../selection-keys";

import { moveHistory } from "./move-history";
import { scheduleHistoryStep } from "./schedule-history-step";
import { TYPED_FIELDS } from "./typed-fields";

import type { SizedHistory } from "../../src";
import type { KeptShape } from "../kept-shapes";

/**
 * The moves of the history, by the letter pressed with Ctrl, or Cmd, read whatever its case, and
 * whether Shift is held, macOS sending Cmd + Shift + Z as a small z: Z undoes, Shift + Z redoes, Y
 * redoes, with Shift or not.
 */
const HISTORY_KEYS: ReadonlyMap<
  string,
  (history: SizedHistory<KeptShape>) => SizedHistory<KeptShape>
> = new Map([
  ["y false", sizedRedoneHistory],
  ["y true", sizedRedoneHistory],
  ["z false", sizedUndoneHistory],
  ["z true", sizedRedoneHistory],
]);

/**
 * Reads the field of the panel a key is typed in, typed in since its value was applied, which
 * keeps the keys of the history (`TYPED_FIELDS`, SP-029's answer 9).
 *
 * @kind procedure
 * @param event - the key pressed
 * @returns the field; undefined outside the fields, or in a field whose value is applied
 */
function readTypedField(event: KeyboardEvent): Element | undefined {
  const field = readKeyField(event);

  return field !== undefined && TYPED_FIELDS.has(field) ? field : undefined;
}

/**
 * Reads the move a key asks of the history (`RULE-interaction-undo-and-redo-in-the-playground-the-keys`): Ctrl + Z undoes, Ctrl + Shift +
 * Z and Ctrl + Y redo, Cmd for Ctrl on macOS (`HISTORY_KEYS`); none when neither Ctrl nor Cmd is
 * held, Alt is, the key is already taken by a listener before this one, its default prevented, or
 * a field typed in keeps it (`readTypedField`).
 *
 * @kind procedure
 * @param event - the key pressed
 * @returns `sizedUndoneHistory` or `sizedRedoneHistory`; undefined for a key the history does not take
 */
function readHistoryMove(
  event: KeyboardEvent,
): ((history: SizedHistory<KeptShape>) => SizedHistory<KeptShape>) | undefined {
  const isLeft = [
    ![event.ctrlKey, event.metaKey].includes(true),
    event.altKey,
    event.defaultPrevented,
    readTypedField(event) !== undefined,
  ].includes(true);

  return isLeft
    ? undefined
    : HISTORY_KEYS.get(`${event.key.toLowerCase()} ${String(event.shiftKey)}`);
}

/**
 * Keeps whether the target of an event is a field typed in (`TYPED_FIELDS`).
 *
 * @kind procedure
 * @param event - an `input`, a `change` or a `focusout`
 * @param isTyped - true from its `input` on, false once applied or left
 */
function writeTypedField(event: Event, isTyped: boolean): void {
  if (event.target instanceof Element) {
    TYPED_FIELDS[isTyped ? "add" : "delete"](event.target);
  }
}

/**
 * Keeps which fields of the panel are typed in (`TYPED_FIELDS`): a field is typed in from its
 * first `input` until its value is applied, Enter or the field left firing `change`, or until the
 * focus leaves it, a value typed back as it was firing no `change`.
 *
 * @kind procedure
 * @param document - page document
 */
function listenToTypedFields(document: Document): void {
  document.addEventListener(
    "input",
    (event) => {
      writeTypedField(event, true);
    },
    { signal: readListenerSignal(document) },
  );

  for (const type of ["change", "focusout"]) {
    document.addEventListener(
      type,
      (event) => {
        writeTypedField(event, false);
      },
      { signal: readListenerSignal(document) },
    );
  }
}

/**
 * Asks a step of the history at each key released outside the panel's fields
 * (`scheduleHistoryStep`), so that a key held is one step from its press to its release (SP-029's
 * answer 3), arrows held together from the first press to the last release; and when
 * the window loses the focus, which releases the arrows held, their releases never heard.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 */
function listenToKeyReleases(document: Document, update: () => void): void {
  document.addEventListener(
    "keyup",
    (event) => {
      if (readKeyField(event) === undefined) {
        scheduleHistoryStep(document, update);
      }
    },
    { signal: readListenerSignal(document) },
  );

  document.defaultView?.addEventListener(
    "blur",
    () => {
      scheduleHistoryStep(document, update);
    },
    { signal: readListenerSignal(document) },
  );
}

/**
 * Listens to the keys of the history (`RULE-interaction-undo-and-redo-in-the-playground-the-keys`): Ctrl + Z undoes, Ctrl + Shift + Z
 * and Ctrl + Y redo, Cmd for Ctrl on macOS, their default prevented (`readHistoryMove`,
 * `moveHistory`); a field of the panel typed in since its value was applied keeps them, any other
 * field leaves them to the drawing (SP-029's answer 9). Heard as the key goes down to its target.
 * A key released outside the fields asks a step (`listenToKeyReleases`).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 */
export function listenToHistoryKeys(document: Document, update: () => void): void {
  listenToTypedFields(document);
  listenToKeyReleases(document, update);

  document.addEventListener(
    "keydown",
    (event) => {
      const move = readHistoryMove(event);

      if (move === undefined) {
        return;
      }

      event.preventDefault();
      moveHistory(document, move, update);
    },
    { capture: true, signal: readListenerSignal(document) },
  );
}
