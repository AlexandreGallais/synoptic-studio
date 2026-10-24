import { parsePanelValue } from "../../src";
import { SHOWN_TEXTS, readShownNumber } from "../applied-values";
import { readListenerSignal } from "../listeners";
import { writeInputValue } from "../page";

import { listenToTypedNumbers } from "./listen-to-typed-numbers";
import { OWN_NUMBER_FIELDS } from "./own-number-fields";
import { readArrowStep } from "./read-arrow-step";
import { readOwnValue } from "./read-own-value";

/**
 * Reads the field acting as a number field of its own an event reached (`OWN_NUMBER_FIELDS`).
 *
 * @kind procedure
 * @param event - an event of the page
 * @returns the field; undefined for any other element
 * @see docs/domain/interaction.md#_2-selector
 */
function readOwnField(event: Event): HTMLInputElement | undefined {
  const { target } = event;

  return target instanceof HTMLInputElement && target.matches(OWN_NUMBER_FIELDS)
    ? target
    : undefined;
}

/**
 * Reads the number a field of its own showed before a text was typed in it (`SHOWN_TEXTS`,
 * `listenToTypedNumbers`), none for a field showing none, an empty Radius showing Mixed.
 *
 * @kind procedure
 * @param field - a field acting as a number field of its own
 * @returns the number; undefined when it showed none
 * @see docs/domain/interaction.md#_2-selector
 */
function readBeforeNumber(field: HTMLInputElement): number | undefined {
  const shown = readShownNumber(SHOWN_TEXTS.get(field) ?? "");

  return Number.isNaN(shown) ? undefined : shown;
}

/**
 * Shows a value in a field of its own; a value refused shows the number it showed again
 * (`readBeforeNumber`), a field without one keeping its text. The number kept is read again at
 * the next key or focus (`listenToTypedNumbers`), once the field's own listeners have shown the value
 * they took, the angle 370 shown 10.
 *
 * @kind procedure
 * @param field - a field acting as a number field of its own
 * @param value - the value it takes; undefined when it is refused
 * @see docs/domain/interaction.md#_2-selector
 */
function showOwnValue(field: HTMLInputElement, value: number | undefined): void {
  const shown = value ?? readBeforeNumber(field);

  if (shown === undefined) {
    return;
  }

  writeInputValue(field.ownerDocument, field.id, shown);
}

/**
 * Reads the value a text typed in a field of its own gives: an integer, relative with a sign
 * (`parsePanelValue`), within its bounds (`readOwnValue`).
 *
 * @kind procedure
 * @param field - a field acting as a number field of its own
 * @returns the value; undefined when it is refused
 * @see docs/domain/interaction.md#_2-selector
 */
function readTypedValue(field: HTMLInputElement): number | undefined {
  const typed = parsePanelValue(field.value);

  return typed === undefined ? undefined : readOwnValue(field, typed, readBeforeNumber(field));
}

/**
 * Stops a value refused in a field of its own, its number shown again, so that no listener of the
 * field applies it; a field that showed none lets it go to them, which read the text as they read
 * a number field's.
 *
 * @kind procedure
 * @param event - a `change` heard on the document before the field
 * @param field - a field acting as a number field of its own
 * @param value - the value it takes; undefined when it is refused
 * @see docs/domain/interaction.md#_2-selector
 */
function stopRefusedValue(event: Event, field: HTMLInputElement, value: number | undefined): void {
  if (value === undefined && readBeforeNumber(field) !== undefined) {
    event.stopPropagation();
  }
}

/**
 * Applies a value typed in a field of its own, at Enter or when it is left (`change`,
 * `readTypedValue`), shown as the number the field's own listeners then read; a value refused
 * shows the number before again and goes no further, nothing applied, unless the field showed
 * none, its own listeners then reading the text as they read a number field's.
 *
 * @kind procedure
 * @param event - a `change` heard on the document before the field
 * @see docs/domain/interaction.md#_2-selector
 */
function applyTypedValue(event: Event): void {
  const field = readOwnField(event);

  if (field === undefined) {
    return;
  }

  const value = readTypedValue(field);

  showOwnValue(field, value);
  stopRefusedValue(event, field, value);
}

/**
 * Shows the value an arrow steps a field of its own to (`showOwnValue`), then applies it as Enter
 * does (`change`); a value refused, past a bound, applies nothing.
 *
 * @kind procedure
 * @param field - a field acting as a number field of its own
 * @param value - the value stepped to; undefined when it is refused
 * @see docs/domain/interaction.md#_2-selector
 */
function applySteppedValue(field: HTMLInputElement, value: number | undefined): void {
  showOwnValue(field, value);

  if (value !== undefined) {
    field.dispatchEvent(new Event("change", { bubbles: true }));
  }
}

/**
 * Steps a field of its own at Up or Down (`readArrowStep`), from the number it showed
 * (`readBeforeNumber`), within its bounds (`readOwnValue`), the key's default prevented, the value
 * applied (`applySteppedValue`) once the panel's listener has opened the burst of arrows
 * (`listenToArrowBursts`), so that it takes no step of its own.
 *
 * @kind procedure
 * @param event - a key pressed, heard on the document after the panel
 * @see docs/domain/interaction.md#_2-selector
 */
function applyArrowValue(event: KeyboardEvent): void {
  const field = readOwnField(event);
  const step = readArrowStep(event);

  if (field === undefined || step === undefined) {
    return;
  }

  event.preventDefault();
  applySteppedValue(field, readOwnValue(field, step, readBeforeNumber(field)));
}

/**
 * Makes the fields acting as number fields of their own act as one shape's (US-087,
 * `OWN_NUMBER_FIELDS`): a value typed applied on Enter or when the field is left, never while it
 * is typed, an integer, relative with a sign from the number shown before it was typed
 * (`listenToTypedNumbers`, `applyTypedValue`), a value refused putting that number back; Up and Down
 * stepping it by 1, or 10 with Shift (`applyArrowValue`). Heard on the document, a value applied
 * before the field's own listeners, which read the number it shows (`readNumber`), an arrow after
 * the panel's, which opens its burst.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_2-selector
 */
export function listenToOwnNumberFields(document: Document): void {
  const signal = readListenerSignal(document);

  listenToTypedNumbers(document);
  document.addEventListener("change", applyTypedValue, { capture: true, signal });
  document.addEventListener("keydown", applyArrowValue, { signal });
}
