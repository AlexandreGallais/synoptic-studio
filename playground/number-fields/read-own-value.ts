import { clamp, modulo, valueWithTyped, wholeValueWithTyped } from "../../src";

import { OWN_FIELD_RANGES } from "./own-field-ranges";

import type { OwnFieldRange } from "./own-field-range";
import type { TypedPanelValue } from "../../src";

/** The range of a field the table does not name: whole numbers without a bound. */
const NO_RANGE: OwnFieldRange = { isTurn: false, isWhole: true, least: -Infinity, most: Infinity };

/**
 * Reads a value within a field's range: taken at its bound beyond it, or, for a turn, gone round
 * it, −10 in the Angle giving 350 and 370 giving 10 (`modulo`).
 *
 * @kind procedure
 * @param value - the value computed
 * @param range - the field's range
 * @returns the value within the range
 * @see docs/domain/interaction.md#_2-selector
 */
function readValueInRange(value: number, range: OwnFieldRange): number {
  const { isTurn, least, most } = range;

  return isTurn ? least + modulo(value - least, most - least) : clamp(value, least, most);
}

/**
 * Reads the value computed from a value applied in a field: from the numbers as typed
 * (`valueWithTyped`), rounded for a field of whole numbers when a fraction is typed
 * (`wholeValueWithTyped`).
 *
 * @kind procedure
 * @param range - the field's range, telling whether it holds whole numbers
 * @param own - the number the field showed before, 0 when it showed none
 * @param typed - the value typed, or an arrow's step, relative
 * @returns the value, before the field's range
 * @see docs/domain/interaction.md#_2-selector
 */
function readComputedValue(range: OwnFieldRange, own: number, typed: TypedPanelValue): number {
  return range.isWhole ? wholeValueWithTyped(own, typed) : valueWithTyped(own, typed);
}

/**
 * Reads the value a field takes from a number it showed: computed (`readComputedValue`), then
 * within the field's range (`OWN_FIELD_RANGES`, `readValueInRange`), a field the table does not
 * name taking whole numbers without a bound.
 *
 * @kind procedure
 * @param field - a field acting as a number field of its own
 * @param own - the number the field showed before
 * @param typed - the value typed, or an arrow's step, relative
 * @returns the value within its range
 * @see docs/domain/interaction.md#_2-selector
 */
function readValueFrom(field: HTMLInputElement, own: number, typed: TypedPanelValue): number {
  const range = OWN_FIELD_RANGES.get(field.id) ?? NO_RANGE;

  return readValueInRange(readComputedValue(range, own, typed), range);
}

/**
 * Reads the value a field acting as a number field of its own takes for a value applied in it
 * (US-087, AUD-046): a value as typed, or a relative one added to the number the field showed
 * before (`valueWithTyped`), rounded when the field holds whole numbers and a fraction is typed
 * (`wholeValueWithTyped`), then within the field's range (`OWN_FIELD_RANGES`,
 * `readValueInRange`): 13 in Corners gives 12, 1.5 in the radius 2, `-20` in the Angle at 10
 * gives 350.
 *
 * @kind procedure
 * @param field - a field acting as a number field of its own
 * @param typed - the value typed, or an arrow's step, relative
 * @param shown - the number the field showed before; undefined when it showed none
 * @returns the value; undefined for a relative value with no number to start from, a Mixed
 * radius's, which each vertex's own takes (`listenToRadius`)
 * @see docs/domain/interaction.md#_2-selector
 */
export function readOwnValue(
  field: HTMLInputElement,
  typed: TypedPanelValue,
  shown: number | undefined,
): number | undefined {
  if (shown === undefined) {
    // A fixed value needs no number to start from.
    return typed.isRelative ? undefined : readValueFrom(field, 0, typed);
  }

  return readValueFrom(field, shown, typed);
}
