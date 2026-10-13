import { turnAbout } from "../../geometry";
import { turnDirection } from "../../math";

import { boxCenter } from "./box-center";
import { positionKeepingPoint } from "./position-keeping-point";

import type { TurnedBox } from "./turned-corners";
import type { Size } from "../../geometry";
import type { Point } from "../../math";

/**
 * Position of a turned shape after its width or height is typed: its turned top-left corner
 * stays where it is drawn, as Figma (Product Owner, F04; `DERIV-turned-shape` step 5). At 0° the
 * position does not change: the top-left of the box is fixed, as in F09.
 *
 * @kind domain
 * @param box - angle, position and size before the edit
 * @param size - width and height typed
 * @returns the top-left of the new box before it turns
 * @see DERIV-turned-shape
 */
export function positionAfterResize(box: TurnedBox, size: Size): Point {
  if (box.angle === 0) {
    return { x: box.x, y: box.y };
  }

  const center = boxCenter(box);
  const corner = turnAbout({ x: box.x, y: box.y }, center, turnDirection(box.angle));

  return positionKeepingPoint({ ...size, angle: box.angle }, { x: 0, y: 0 }, corner);
}
