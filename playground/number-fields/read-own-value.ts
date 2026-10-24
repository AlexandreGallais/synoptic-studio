import type { TypedPanelValue } from "../../src";

/**
 * Reads the value a field acting as a number field of its own takes for a value applied in it
 * (US-087): an integer as typed, or a relative one added to the number the field showed before
 * (`SHOWN_NUMBERS`), within the field's bounds, both kept (`data-min`, `data-max`).
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
  return typed.isRelative ? shown : field.valueAsNumber;
}
