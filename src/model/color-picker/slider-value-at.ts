import { clamp } from "../../math";

/**
 * The value a slider holds at a point along it: its largest value times the distance from its
 * left end, out of its length, the multiplication first; a point beyond an end, during a drag,
 * counts at that end; a slider of length 0 holds 0 (`DERIV-color-spaces`, step 10). On a hue
 * slider 200 long, 100 is 180°.
 *
 * Formula: v = clamp(M · x / w, 0, M), 0 when w = 0.
 *
 * @kind domain
 * @param position - the distance from the left end
 * @param length - how long the slider is, from end to end
 * @param maximum - the value at its right end, `M`: 360 for the hue, 100 for a percent
 * @returns the value, from 0 to `maximum`
 * @see DERIV-color-spaces
 */
export function sliderValueAt(position: number, length: number, maximum: number): number {
  return length === 0 ? 0 : clamp((maximum * position) / length, 0, maximum);
}
