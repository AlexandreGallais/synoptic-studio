import { SHOWN_TEXTS, readShownNumber, writeFieldText } from "../applied-values";
import { writeInputValue } from "../page";

import { readOwnValue } from "./read-own-value";

import type { TypedPanelValue } from "../../src";

/**
 * Reads the number a field of its own showed before a text was typed in it (`SHOWN_TEXTS`,
 * `listenToTypedNumbers`), none for a field showing none, an empty Radius showing Mixed.
 *
 * @kind procedure
 * @param field - a field acting as a number field of its own
 * @returns the number; undefined when it showed none
 * @see RULE-interaction-2-selector
 */
function readBeforeNumber(field: HTMLInputElement): number | undefined {
  const shown = readShownNumber(SHOWN_TEXTS.get(field) ?? "");

  return Number.isNaN(shown) ? undefined : shown;
}

/**
 * Reads a relative value as typed, its sign written, `+1` or `-10`, `+0` for a relative 0.
 *
 * @kind procedure
 * @param typed - a value relative, typed or an arrow's step
 * @returns its text
 * @see RULE-interaction-2-selector
 */
function readRelativeText(typed: TypedPanelValue): string {
  return typed.value >= 0 ? `+${String(typed.value)}` : String(typed.value);
}

/**
 * Shows in a field of its own the value a value applied gives, from the number it showed and
 * within its range (`readOwnValue`), which the field's own listeners then read; a relative value
 * with no number shown, in a Mixed radius, is shown as typed (`readRelativeText`), each vertex's
 * own taking it (`listenToRadius`).
 *
 * @kind procedure
 * @param field - a field acting as a number field of its own
 * @param typed - the value typed, or an arrow's step, relative
 * @see RULE-interaction-2-selector
 */
export function writeOwnValue(field: HTMLInputElement, typed: TypedPanelValue): void {
  const value = readOwnValue(field, typed, readBeforeNumber(field));
  const { id, ownerDocument } = field;

  if (value === undefined) {
    writeFieldText(ownerDocument, id, readRelativeText(typed));

    return;
  }

  writeInputValue(ownerDocument, id, value);
}
