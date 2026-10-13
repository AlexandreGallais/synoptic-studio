import { stretchToSize } from "../box";

import { stretchScale } from "./stretch-scale";
import { unitStretchedPolygon } from "./unit-stretched-polygon";

import type { Point } from "../../math";
import type { Size } from "../box";

/**
 * Stretched polygon of `n` corners filling a `width × height` box from the origin: the unit
 * polygon of rule C for the ratio of the box, each axis scaled to the box, the top-left fixed, so
 * that it touches the four sides, within a rounding error of the box (one unit in the last place:
 * the span of the unit polygon is not an integer) (`DERIV-stretched-polygon` step 6). Its sides are equal while its
 * corners keep half their regular turn; past that, the limit polygon is stretched. A size of 0
 * puts every point on the side at 0.
 *
 * @kind geometry
 * @param corners - number of corners `n`, an integer from 3 to 12
 * @param size - width and height of the box, `≥ 0`
 * @returns the `n` vertices in drawing order, clockwise on screen from the right end of the base
 * @see DERIV-stretched-polygon
 */
export function stretchedPolygon(corners: number, size: Size): readonly Point[] {
  const scale = stretchScale(corners, size.height / size.width);

  return stretchToSize(unitStretchedPolygon(corners, scale), size);
}
