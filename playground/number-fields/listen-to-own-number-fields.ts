import { parsePanelValue } from "../../src";
import { SHOWN_TEXTS, readShownNumber, resetShownText, writeFieldText } from "../applied-values";
import { readListenerSignal } from "../listeners";
import { writeInputValue } from "../page";

import { listenToTypedNumbers } from "./listen-to-typed-numbers";
import { OWN_NUMBER_FIELDS } from "./own-number-fields";
import { readArrowStep } from "./read-arrow-step";
import { readNumberField } from "./read-number-field";
import { readOwnValue } from "./read-own-value";

import type { TypedPanelValue } from "../../src";

/**
 * Reads the field acting as a number field of its own an event reached (`OWN_NUMBER_FIELDS`).
 *
 * @kind procedure
 * @param event - an event of the page
 * @returns the field; undefined for any other element
 * @see docs/domain/interaction.md#_2-selector
 */
function readOwnField(event: Event): HTMLInputElement | undefined {
  const field = readNumberField(event.target);

  return field?.matches(OWN_NUMBER_FIELDS) === true ? field : undefined;
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
 * Stops a text that is no number typed in a field of its own, the text it showed before put back
 * (`resetShownText`), a Mixed radius's empty one included, so that no listener of the field
 * applies it; a field whose text the page never read, typed by a script without the focus nor a
 * key, lets it go to them, which read it as they read a number field's.
 *
 * @kind procedure
 * @param event - a `change` heard on the document before the field
 * @param field - a field acting as a number field of its own
 * @see docs/domain/interaction.md#_2-selector
 */
function stopRefusedText(event: Event, field: HTMLInputElement): void {
  if (!SHOWN_TEXTS.has(field)) {
    return;
  }

  resetShownText(field.ownerDocument, field.id);
  event.stopPropagation();
}

/**
 * Reads a relative value as typed, its sign written, `+1` or `-10`.
 *
 * @kind procedure
 * @param typed - a value relative, typed or an arrow's step
 * @returns its text
 * @see docs/domain/interaction.md#_2-selector
 */
function readRelativeText(typed: TypedPanelValue): string {
  return typed.value >= 0 ? `+${String(typed.value)}` : String(typed.value);
}

/**
 * Shows in a field of its own the value a value applied gives, within its range
 * (`readOwnValue`), which the field's own listeners then read; a relative value with no number
 * shown, in a Mixed radius, is shown as typed (`readRelativeText`), each vertex's own taking it.
 *
 * @kind procedure
 * @param field - a field acting as a number field of its own
 * @param typed - the value typed, or an arrow's step, relative
 * @see docs/domain/interaction.md#_2-selector
 */
function writeOwnValue(field: HTMLInputElement, typed: TypedPanelValue): void {
  const value = readOwnValue(field, typed, readBeforeNumber(field));
  const { id, ownerDocument } = field;

  if (value === undefined) {
    writeFieldText(ownerDocument, id, readRelativeText(typed));

    return;
  }

  writeInputValue(ownerDocument, id, value);
}

/**
 * Applies the text typed in a field of its own: a number, relative with a sign
 * (`parsePanelValue`), shown as the value it gives (`writeOwnValue`); a text that is no number
 * refused (`stopRefusedText`).
 *
 * @kind procedure
 * @param event - a `change` heard on the document before the field
 * @param field - a field acting as a number field of its own
 * @see docs/domain/interaction.md#_2-selector
 */
function applyTypedText(event: Event, field: HTMLInputElement): void {
  const typed = parsePanelValue(field.value);

  if (typed === undefined) {
    stopRefusedText(event, field);

    return;
  }

  writeOwnValue(field, typed);
}

/**
 * Applies a value typed in a field of its own, at Enter or when it is left (`change`,
 * `applyTypedText`).
 *
 * @kind procedure
 * @param event - a `change` heard on the document before the field
 * @see docs/domain/interaction.md#_2-selector
 */
function applyTypedValue(event: Event): void {
  const field = readOwnField(event);

  if (field !== undefined) {
    applyTypedText(event, field);
  }
}

/**
 * Steps a field of its own at Up or Down (`readArrowStep`), from the number it showed, within its
 * range (`writeOwnValue`), the key's default prevented, then applies it as Enter does (`change`),
 * once the panel's listener has opened the burst of arrows (`listenToArrowBursts`), so that it
 * takes no step of its own; a field showing no number, a Mixed radius, is given the step as
 * typed, `+1` or `-10`, which each vertex's own takes.
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
  writeOwnValue(field, step);
  field.dispatchEvent(new Event("change", { bubbles: true }));
}

/**
 * Makes the fields acting as number fields of their own act as one shape's (US-087, AUD-046,
 * `OWN_NUMBER_FIELDS`): a value typed applied on Enter or when the field is left, never while it
 * is typed, a number, relative with a sign from the number shown before it was typed
 * (`listenToTypedNumbers`, `applyTypedValue`), taken within the field's range, a text that is no
 * number putting that number back; Up and Down stepping it by 1, or 10 with Shift
 * (`applyArrowValue`). Heard on the document, a value applied before the field's own listeners,
 * which read the number it shows (`readNumber`), an arrow after the panel's, which opens its
 * burst.
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
