import { readListenerSignal } from "../listeners";
import { HELD_SPACES } from "../view";

import { readKeyField } from "./read-key-field";

/**
 * Presses `Space` outside the panel's fields: held from then, its default prevented; a key of a
 * field, or another key, changes nothing.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the key pressed
 * @see docs/domain/interaction.md#_1-tools
 */
function pressSpace(document: Document, event: KeyboardEvent): void {
  if (!(event.key === " " && readKeyField(event) === undefined)) {
    return;
  }

  event.preventDefault();
  HELD_SPACES.add(document);
}

/**
 * Removes `Space` held at any key or pointer event that carries Cmd, its default left: a key
 * released while Cmd is held sends no `keyup` on macOS (`REF-WEBKIT-CMD-KEYUP`), Cmd + Space is the
 * system's (`REF-APPLE-SHORTCUTS`), which may take Cmd's `keyup` too, while Cmd's own `keydown`
 * carries it (`REF-UIEVENTS-KEYS`), so that Space let go under Cmd no longer makes the next drag
 * pan (US-091); an event without Cmd changes nothing.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the key pressed, or the pointer moved or pressed
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
function removeSpaceUnderCommand(document: Document, event: KeyboardEvent | PointerEvent): void {
  if (event.metaKey) {
    HELD_SPACES.delete(document);
  }
}

/**
 * Removes `Space` held at its release, wherever the focus is, its default prevented outside the
 * panel's fields; another key changes nothing.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the key released
 * @see docs/domain/interaction.md#_1-tools
 */
function removeHeldSpace(document: Document, event: KeyboardEvent): void {
  if (event.key !== " ") {
    return;
  }

  HELD_SPACES.delete(document);

  if (readKeyField(event) === undefined) {
    event.preventDefault();
  }
}

/**
 * Listens to `Space` outside the panel's fields (`readKeyField`): held, a drag with the main button
 * pans the view (`HELD_SPACES`, SP-030's answer 2), its press and its release prevented so that the
 * page does not scroll nor a button of the toolbar take it; typed in a field, it is the field's
 * (`interaction.md` §1). Its release ends it wherever the focus is, and so do any key or pointer
 * event carrying Cmd, heard before the canvas's press, a release under Cmd never heard on macOS
 * (US-091), and the window losing the focus, a release in another window never heard (AUD-044).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_1-tools
 */
export function listenToSpace(document: Document): void {
  document.addEventListener(
    "keydown",
    (event) => {
      // Space pressed with Cmd, then let go at once: Cmd + Space is the system's.
      pressSpace(document, event);
      removeSpaceUnderCommand(document, event);
    },
    { signal: readListenerSignal(document) },
  );

  for (const type of ["pointerdown", "pointermove"] as const) {
    document.addEventListener(
      type,
      (event) => {
        removeSpaceUnderCommand(document, event);
      },
      { capture: true, signal: readListenerSignal(document) },
    );
  }

  document.addEventListener(
    "keyup",
    (event) => {
      removeHeldSpace(document, event);
    },
    { signal: readListenerSignal(document) },
  );

  document.defaultView?.addEventListener(
    "blur",
    () => {
      HELD_SPACES.delete(document);
    },
    { signal: readListenerSignal(document) },
  );
}
