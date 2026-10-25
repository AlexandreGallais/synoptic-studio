import { closeColorPicker, readOpenSwatch, toggleColorPicker } from "../color-picker";
import { readListenerSignal } from "../listeners";

import { readPickerAreas } from "./read-picker-areas";

/**
 * Closes the open picker on Escape, its swatch taking the focus back.
 *
 * @kind procedure
 * @param document - page document
 * @param key - the key pressed, as `KeyboardEvent.key` names it
 * @see US-035
 */
function closePickerOnEscape(document: Document, key: string): void {
  const swatch = readOpenSwatch(document);

  if (key !== "Escape" || swatch === undefined) {
    return;
  }

  closeColorPicker(document);
  swatch.focus();
}

/**
 * Closes the open picker on a click outside it and its color's fields.
 *
 * @kind procedure
 * @param document - page document
 * @param path - the element clicked and those holding it, up to the window
 * @see US-035
 */
function closePickerOutside(document: Document, path: readonly EventTarget[]): void {
  if (readPickerAreas(document).every((area) => !path.includes(area))) {
    closeColorPicker(document);
  }
}

/**
 * Listens to what opens and closes the picker: a click on a swatch opens it, a second click
 * closes it, as Escape does, and a click outside it and its color's fields
 * (`docs/domain/interaction.md` §5).
 *
 * @kind procedure
 * @param document - page document
 * @see US-035
 */
export function listenToPickerClosing(document: Document): void {
  for (const swatch of document.querySelectorAll<HTMLElement>(".swatch")) {
    swatch.addEventListener("click", () => {
      toggleColorPicker(document, swatch);
    });
  }

  document.addEventListener(
    "keydown",
    (event) => {
      closePickerOnEscape(document, event.key);
    },
    { signal: readListenerSignal(document) },
  );

  document.addEventListener(
    "pointerdown",
    (event) => {
      closePickerOutside(document, event.composedPath());
    },
    { signal: readListenerSignal(document) },
  );
}
