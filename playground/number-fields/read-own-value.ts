import { isBetween } from "../../src";

import type { TypedPanelValue } from "../../src";

/**
 * Reads the number a value applied in a field gives: a fixed one as it is, a relative one added
 * to the number the field showed before.
 *
 * @kind procedure
 * @param typed - the value typed, or an arrow's step, relative
 * @param shown - the number the field showed before; undefined when it showed none
 * @returns the number; `NaN` for a relative value with no number to start from
 * @see docs/domain/interaction.md#_2-selector
 */
function readAppliedNumber(typed: TypedPanelValue, shown: number | undefined): number {
  if (!typed.isRelative) {
    return typed.value;
  }

  return (shown ?? NaN) + typed.value;
}

/**
 * Reads the value a field acting as a number field of its own takes for a value applied in it
 * (US-087): an integer as typed, or a relative one added to the number the field showed before
 * (`SHOWN_NUMBERS`, `readAppliedNumber`), within the field's bounds, both kept (`data-min`,
 * `data-max`, `isBetween`), a bound the field lacks taken as none.
 *
 * @kind procedure
 * @param field - a field acting as a number field of its own, its bounds in its data
 * @param typed - the value typed, or an arrow's step, relative
 * @param shown - the number the field showed before; undefined when it showed none
 * @returns the value; undefined when it is refused, past a bound, or relative to no number
 * @see docs/domain/interaction.md#_2-selector
 */
export function readOwnValue(
  field: HTMLInputElement,
  typed: TypedPanelValue,
  shown: number | undefined,
): number | undefined {
  const value = readAppliedNumber(typed, shown);
  const { max = "Infinity", min = "-Infinity" } = field.dataset;

  return isBetween(value, Number(min), Number(max)) ? value : undefined;
}
