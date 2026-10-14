import { halfSize, turnAbout } from "../../geometry";
import { subtract, turnDirection } from "../../math";

import type { Size } from "../../geometry";
import type { Point } from "../../math";

/** Angle and size of a box whose position is sought. */
export type TurnedSize = Size & {
  /** Angle in degrees, from 0 to 359, clockwise on screen. */
  readonly angle: number;
};

/**
 * Position of a turned box such that one of its points is drawn at a given point of the SVG:
 * `b = P − (k + R(l − k))`, `k` the half size of the box, `l` the point relative to the top-left
 * of the box before it turns, `R` the turn about the center (`DERIV-turned-shape` step 5). The
 * shape is rigid under the turn: every other point keeps its place relative to that one.
 *
 * @kind domain
 * @param box - angle, width and height of the box
 * @param local - the point, relative to the top-left of the box before it turns
 * @param drawn - where it must be drawn, in the SVG
 * @returns the top-left of the box before it turns
 * @see DERIV-turned-shape
 */
export function positionKeepingPoint(box: TurnedSize, local: Point, drawn: Point): Point {
  const turned = turnAbout(local, halfSize(box), turnDirection(box.angle));

  return subtract(drawn, turned);
}
