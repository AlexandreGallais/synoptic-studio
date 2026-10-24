import { nearestInteger } from "../../math";

import { valueWithTyped } from "./value-with-typed";

import type { TypedPanelValue } from "./typed-panel-value";

/**
 * A value once a value is typed in a field of integers (AUD-046): computed from the numbers as
 * typed (`valueWithTyped`), then rounded to the nearest integer, a tie rounded up, as CSS rounds
 * (`nearestInteger`, Q45), when the value typed has a fraction; `1.5` gives 2, 10 and `+1.5` give
 * 12, 10 and `-2.5` give 8. An integer typed keeps a fraction of the own value, a converted
 * polygon's size (Q27) or a turned shape's X: 86.60254 and `+10` give 96.60254.
 *
 * @kind domain
 * @param own - the value the field showed, or a shape's own in it
 * @param typed - the value typed, or an arrow's step, relative
 * @returns the value, an integer for a fraction typed
 * @see docs/domain/interaction.md#_2-selector
 */
export function wholeValueWithTyped(own: number, typed: TypedPanelValue): number {
  const value = valueWithTyped(own, typed);

  return Number.isSafeInteger(typed.value) ? value : nearestInteger(value);
}
