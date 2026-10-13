import { distanceFromSide } from "./distance-from-side";
import { filletCentre } from "./fillet-centre";
import { firstCrossing } from "./first-crossing";
import { footOnSide } from "./foot-on-side";

import type { FilletSide } from "./fillet-side";
import type { Point } from "../../math";

/** Where a fillet touches its two sides. */
export type FilletTangency = {
  /** Its centre. */
  readonly centre: Point;
  /** Parameter of the tangent point on the side before. */
  readonly before: number;
  /** Parameter of the tangent point on the side after. */
  readonly after: number;
};

/**
 * Where a fillet of a radius touches a corner's two sides: its centre runs along the parallel
 * curve of the side after, from the corner, until it is first at the radius from the side before
 * (`DERIV-fillet-beside-curve` steps 3, 7 and 10).
 *
 * @kind geometry
 * @param sides - the side before, then the side after, oriented
 * @param radius - the radius tried
 * @returns its centre and its two tangent points' parameters, or `undefined` when the centre never
 * gets that far from the side before
 * @see DERIV-fillet-beside-curve
 */
export function filletTangency(
  sides: readonly [FilletSide, FilletSide],
  radius: number,
): FilletTangency | undefined {
  const [before, after] = sides;
  const tAfter = firstCrossing(
    (t) => distanceFromSide(before, filletCentre(after, radius, t), before.limit) < radius,
    after.limit,
  );

  if (tAfter === undefined) {
    return undefined;
  }

  const centre = filletCentre(after, radius, tAfter);

  return { after: tAfter, before: footOnSide(before, centre, before.limit), centre };
}
