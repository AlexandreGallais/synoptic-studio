/**
 * Where a value sits along a slider, from its left end: its length times the value, out of the
 * largest value, the multiplication first; `sliderValueAt` read back (`DERIV-color-spaces`,
 * step 10). On a hue slider 360 long, 30.1° sits at 30.1.
 *
 * Formula: x = w · v / M.
 *
 * @kind domain
 * @param value - the value, from 0 to `maximum`
 * @param length - how long the slider is, from end to end
 * @param maximum - the value at its right end, `M`, > 0
 * @returns the distance from the left end, from 0 to `length`
 * @see DERIV-color-spaces
 */
export function sliderPositionOf(value: number, length: number, maximum: number): number {
  return (length * value) / maximum;
}
