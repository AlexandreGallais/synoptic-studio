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
 * Removes `Space` held as Cmd is released, its default left: macOS sends no `keyup` for a key
 * released while Cmd is held (`REF-WEBKIT-CMD-KEYUP`, `REF-MOZILLA-CMD-KEYUP`), so that Space
 * let go under Cmd no longer makes the next drag pan (US-090); another key changes nothing.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the key released
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
function removeSpaceUnderCommand(document: Document, event: KeyboardEvent): void {
  if (event.key === "Meta") {
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
 * (`interaction.md` §1). Its release ends it wherever the focus is, and so do Cmd's release, a
 * release under Cmd never heard on macOS (US-090), and the window losing the focus, a release in
 * another window never heard (AUD-044).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_1-tools
 */
export function listenToSpace(document: Document): void {
  document.addEventListener(
    "keydown",
    (event) => {
      pressSpace(document, event);
    },
    { signal: readListenerSignal(document) },
  );

  document.addEventListener(
    "keyup",
    (event) => {
      removeHeldSpace(document, event);
      removeSpaceUnderCommand(document, event);
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
