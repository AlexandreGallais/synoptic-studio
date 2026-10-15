import { PERCENT_SLIDER_RANGE } from "./percent-slider-range";
import { sliderPositionOf } from "./slider-position-of";

import type { PlaneValues } from "./plane-values-at";
import type { Size } from "../../geometry";
import type { Point } from "../../math";

/**
 * Where a saturation and a lightness or brightness sit on a picker's plane, from its top-left
 * corner, y down: `planeValuesAt` read back (`DERIV-color-spaces`, step 10). On a plane 200 × 100,
 * 100 % and 50 % sit at the middle of the right edge, (200, 50).
 *
 * Formula: x = w · S / 100, y = h − h · L / 100.
 *
 * @kind domain
 * @param values - the saturation and the lightness or brightness, in percent
 * @param size - the plane's width and height
 * @returns where they sit, from the top-left corner, y down
 * @see DERIV-color-spaces
 */
export function planePointOf(values: PlaneValues, size: Size): Point {
  const { maximum } = PERCENT_SLIDER_RANGE;

  return {
    x: sliderPositionOf(values.saturation, size.width, maximum),
    // The distance up from the bottom edge, taken from the height.
    y: size.height - sliderPositionOf(values.level, size.height, maximum),
  };
}
