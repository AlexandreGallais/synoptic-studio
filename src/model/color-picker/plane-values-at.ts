import { PERCENT_SLIDER_RANGE } from "./percent-slider-range";
import { sliderValueAt } from "./slider-value-at";

import type { PickedColor } from "./picked-color";
import type { Size } from "../../geometry";
import type { Point } from "../../math";

/** The saturation and lightness or brightness a point of the plane stands for. */
export type PlaneValues = Pick<PickedColor, "level" | "saturation">;

/**
 * The saturation and lightness or brightness at a point of a picker's plane: the saturation
 * across, from 0 at the left to 100 % at the right, the lightness or brightness up, from 0 at the
 * bottom to 100 % at the top, as a slider each (`sliderValueAt`), a point outside counted at the
 * nearest edge (`DERIV-color-spaces`, step 10). The middle of the right edge of a plane 200 × 100
 * is 100 %, 50 %.
 *
 * Formula: S = 100 · x / w, L or V = 100 · (h − y) / h, each from 0 to 100.
 *
 * @kind domain
 * @param point - the point, from the plane's top-left corner, y down
 * @param size - the plane's width and height
 * @returns the saturation and the lightness or brightness, in percent
 * @see DERIV-color-spaces
 */
export function planeValuesAt(point: Point, size: Size): PlaneValues {
  const { maximum } = PERCENT_SLIDER_RANGE;

  return {
    // The distance up from the bottom edge: 100 · (1 − y / h) = 100 · (h − y) / h.
    level: sliderValueAt(size.height - point.y, size.height, maximum),
    saturation: sliderValueAt(point.x, size.width, maximum),
  };
}
