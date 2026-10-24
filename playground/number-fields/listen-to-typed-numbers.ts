import { SHOWN_TEXTS, TYPED_FIELDS } from "../applied-values";
import { readListenerSignal } from "../listeners";

import { readNumberField } from "./read-number-field";

/**
 * Writes the text a field acting as a number field shows as it gets a key or the focus, while no
 * text typed waits in it, as the value applied (`SHOWN_TEXTS`), so that a text typed and not
 * applied keeps the one before: `-5` typed in the Angle at 30 starts from 30 at each of its keys.
 *
 * @kind procedure
 * @param event - a key pressed, or the focus given
 * @see docs/domain/interaction.md#_2-selector
 */
function writeShownText(event: Event): void {
  const field = readNumberField(event.target);

  if (field !== undefined && !TYPED_FIELDS.has(field)) {
    SHOWN_TEXTS.set(field, field.value);
  }
}

/**
 * Marks a field acting as a number field as holding a text typed and not applied
 * (`TYPED_FIELDS`), so that the page reads the value applied until one is (`readShownText`).
 *
 * @kind procedure
 * @param event - a text typed, `input`
 * @see docs/domain/interaction.md#_2-selector
 */
function writeTypedField(event: Event): void {
  const field = readNumberField(event.target);

  if (field !== undefined) {
    TYPED_FIELDS.add(field);
  }
}

/**
 * Clears the mark of a text typed as a field acting as a number field applies it (`change`),
 * before the field's own listeners, which then read what it shows.
 *
 * @kind procedure
 * @param event - a value applied, `change`
 * @see docs/domain/interaction.md#_2-selector
 */
function clearTypedField(event: Event): void {
  const field = readNumberField(event.target);

  if (field !== undefined) {
    TYPED_FIELDS.delete(field);
  }
}

/**
 * Writes the text a field acting as a number field shows as it gets the focus as the value
 * applied (`writeShownText`), a text typed and left without a change forgotten
 * (`clearTypedField`).
 *
 * @kind procedure
 * @param event - the focus given
 * @see docs/domain/interaction.md#_2-selector
 */
function writeFocusedText(event: Event): void {
  clearTypedField(event);
  writeShownText(event);
}

/**
 * Keeps, for every field acting as a number field (`NUMBER_FIELDS`), the value applied apart from
 * a text typed and not applied (AUD-046): the text it shows read at the focus and at each key
 * while no text typed waits in it (`writeShownText`), never at a key typing in it, the field
 * marked typed at its `input` until a value is applied or the page writes one, so that a relative
 * value starts from the value applied and a redraw meanwhile reads it (`readShownText`). Heard on
 * the document, before the fields' own listeners.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_2-selector
 */
export function listenToTypedNumbers(document: Document): void {
  const signal = readListenerSignal(document);
  const options = { capture: true, signal };

  document.addEventListener("focusin", writeFocusedText, options);
  document.addEventListener("keydown", writeShownText, options);
  document.addEventListener("input", writeTypedField, options);
  document.addEventListener("change", clearTypedField, options);
}
