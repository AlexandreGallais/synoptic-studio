import { isBrowserKey, nudgeShift } from "../../src";
import { readListenerSignal } from "../listeners";

import { HELD_ARROWS } from "./held-arrows";
import { readHeldArrows } from "./read-held-arrows";
import { readKeyField } from "./read-key-field";

/**
 * Holds an arrow pressed outside the panel's fields (`readKeyField`), once; another key, a key of
 * a field, which keeps it, or an arrow with Ctrl, Cmd or Alt held, the browser's (`isBrowserKey`),
 * whose `keyup` macOS never sends with Cmd, changes nothing.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the key pressed
 * @see docs/domain/interaction.md#_2-selector
 */
function pressHeldArrow(document: Document, event: KeyboardEvent): void {
  const held = readHeldArrows(document);

  const isLeft = [
    nudgeShift(event.key, false) === undefined,
    isBrowserKey(event),
    readKeyField(event) !== undefined,
    held.includes(event.key),
  ].includes(true);

  if (!isLeft) {
    HELD_ARROWS.set(document, [...held, event.key]);
  }
}

/**
 * Releases every arrow held at any key or pointer event that carries Cmd: a key released while
 * Cmd is held sends no `keyup` on macOS (`REF-WEBKIT-CMD-KEYUP`, `REF-MOZILLA-CMD-KEYUP`), and a
 * system shortcut may take Cmd's own `keyup`, while Cmd's `keydown` carries it
 * (`REF-UIEVENTS-KEYS`), as `Space` (US-091, AUD-049); an event without Cmd changes nothing.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the key pressed, or the pointer moved or pressed
 * @see docs/domain/interaction.md#_2-selector
 */
function removeArrowsUnderCommand(document: Document, event: KeyboardEvent | PointerEvent): void {
  if (event.metaKey) {
    HELD_ARROWS.delete(document);
  }
}

/**
 * Releases the arrow a key released was, wherever the focus is.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the key released
 * @see docs/domain/interaction.md#_2-selector
 */
function removeReleasedArrow(document: Document, event: KeyboardEvent): void {
  HELD_ARROWS.set(
    document,
    readHeldArrows(document).filter((key) => key !== event.key),
  );
}

/**
 * Listens to the arrows held outside the panel's fields (US-081, `HELD_ARROWS`): an arrow's
 * `keydown` without Ctrl, Cmd or Alt holds it (`pressHeldArrow`), its `keyup` releases it wherever
 * the focus is (`removeReleasedArrow`), any key or pointer event carrying Cmd releases them all,
 * an arrow still down holding again at its next repeat without Cmd (`removeArrowsUnderCommand`),
 * and the window losing the focus releases them all, a release in another window never heard, as
 * `Space` (AUD-044); the page being mounted, none is held. Heard before the arrows nudge, so that
 * a key nudges by every arrow held.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_2-selector
 */
export function listenToHeldArrows(document: Document): void {
  HELD_ARROWS.delete(document);

  document.addEventListener(
    "keydown",
    (event) => {
      removeArrowsUnderCommand(document, event);
      pressHeldArrow(document, event);
    },
    { signal: readListenerSignal(document) },
  );

  for (const type of ["pointerdown", "pointermove"] as const) {
    document.addEventListener(
      type,
      (event) => {
        removeArrowsUnderCommand(document, event);
      },
      { capture: true, signal: readListenerSignal(document) },
    );
  }

  document.addEventListener(
    "keyup",
    (event) => {
      removeReleasedArrow(document, event);
    },
    { signal: readListenerSignal(document) },
  );

  document.defaultView?.addEventListener(
    "blur",
    () => {
      HELD_ARROWS.delete(document);
    },
    { signal: readListenerSignal(document) },
  );
}
