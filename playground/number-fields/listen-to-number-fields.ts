import { parsePanelValue } from "../../src";
import { readListenerSignal } from "../listeners";

import { applyOwnText } from "./apply-own-text";
import { listenToTypedNumbers } from "./listen-to-typed-numbers";
import { NUMBER_FIELD_APPLIES } from "./number-field-applies";
import { readArrowStep } from "./read-arrow-step";
import { readNumberField } from "./read-number-field";
import { STEPPED_CHANGES } from "./stepped-changes";
import { writeOwnValue } from "./write-own-value";

import type { TypedPanelValue } from "../../src";

/**
 * Reads what a field whose value sets the shapes does with a value applied in it
 * (`NUMBER_FIELD_APPLIES`).
 *
 * @kind procedure
 * @param field - a field acting as a number field
 * @returns its apply; undefined for a field of its own
 * @see RULE-interaction-2-selector
 */
function readFieldApply(
  field: HTMLInputElement,
): ((typed: TypedPanelValue | undefined, isStep: boolean) => void) | undefined {
  return NUMBER_FIELD_APPLIES.get(field.ownerDocument)?.get(field.id);
}

/**
 * Applies the text typed in a field acting as a number field: to the shapes it sets, read as a
 * number (`parsePanelValue`), or, for a field of its own, shown within its range before its own
 * listeners read it (`applyOwnText`).
 *
 * @kind procedure
 * @param event - a `change` heard on the document before the field
 * @param field - a field acting as a number field
 * @see RULE-interaction-2-selector
 */
function applyFieldText(event: Event, field: HTMLInputElement): void {
  const apply = readFieldApply(field);

  if (apply === undefined) {
    applyOwnText(event, field);

    return;
  }

  apply(parsePanelValue(field.value), false);
}

/**
 * Applies a value typed in a field acting as a number field, at Enter or when it is left
 * (`change`, `applyFieldText`); the `change` of an arrow's step, its value applied already
 * (`STEPPED_CHANGES`), goes to the field's own listeners as it is, where reading it again as
 * typed made a negative one relative (US-088).
 *
 * @kind procedure
 * @param event - a `change` heard on the document before the field
 * @see RULE-interaction-2-selector
 */
function applyTypedValue(event: Event): void {
  const field = readNumberField(event.target);

  if (field !== undefined && !STEPPED_CHANGES.has(event)) {
    applyFieldText(event, field);
  }
}

/**
 * Applies an arrow's step, relative, to a field acting as a number field: to the shapes it sets,
 * told it is a step, or, for a field of its own, shown within its range (`writeOwnValue`), then applied as Enter does
 * (`change`), its own listeners reading it, that `change` marked as applied already
 * (`STEPPED_CHANGES`), so that `-14` shown at Up from −15 is never read again as typed.
 *
 * @kind procedure
 * @param field - a field acting as a number field
 * @param step - the arrow's step, `+1` or `-10`
 * @see RULE-interaction-2-selector
 */
function applySteppedValue(field: HTMLInputElement, step: TypedPanelValue): void {
  const apply = readFieldApply(field);

  if (apply !== undefined) {
    apply(step, true);

    return;
  }

  const change = new Event("change", { bubbles: true });

  writeOwnValue(field, step);
  STEPPED_CHANGES.add(change);
  field.dispatchEvent(change);
}

/**
 * Steps a field acting as a number field at Up or Down (`readArrowStep`), by 1, or 10 with Shift,
 * the key's default prevented (`applySteppedValue`), once the panel's listener has opened the
 * burst of arrows (`listenToArrowBursts`), so that it takes no step of its own; a key with Ctrl,
 * Cmd or Alt, or another key, is left to the field.
 *
 * @kind procedure
 * @param event - a key pressed, heard on the document after the panel
 * @see RULE-interaction-2-selector
 */
function applyArrowValue(event: KeyboardEvent): void {
  const field = readNumberField(event.target);
  const step = readArrowStep(event);

  if (field === undefined || step === undefined) {
    return;
  }

  event.preventDefault();
  applySteppedValue(field, step);
}

/**
 * Makes every field acting as a number field act under one rule (US-085, US-087, AUD-046,
 * `NUMBER_FIELDS`), one mechanism heard on the document whose apply alone differs, the shapes'
 * (`listenToNumberField`) or the field's own: a value typed applied on Enter or when the field is
 * left, never while it is typed, the value applied kept apart from the text typed
 * (`listenToTypedNumbers`), a number, relative with a sign (`applyTypedValue`); Up and Down
 * stepping it by 1, or 10 with Shift (`applyArrowValue`). A value is applied before the field's
 * own listeners, which read the number it shows (`readNumber`), an arrow after the panel's,
 * which opens its burst.
 *
 * @kind procedure
 * @param document - page document
 * @see RULE-interaction-2-selector
 */
export function listenToNumberFields(document: Document): void {
  const signal = readListenerSignal(document);

  listenToTypedNumbers(document);
  document.addEventListener("change", applyTypedValue, { capture: true, signal });
  document.addEventListener("keydown", applyArrowValue, { signal });
}
