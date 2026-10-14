import { crossings, distanceFromSide, footOnSide } from "../fillet-side";

import { filletCentre } from "./fillet-centre";
import { isValidTangency } from "./is-valid-tangency";

import type { FilletSide } from "../fillet-side";
import type { FilletCandidate } from "./is-valid-tangency";
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
 * The candidates where a fillet touches two sides: its centre runs along the parallel curve of one
 * side until it is the radius from the other, at each crossing, closest to the corner first
 * (`DERIV-fillet-beside-curve` steps 3, 7 and 10).
 *
 * @kind geometry
 * @param sides - the side measured from, then the side run along, oriented
 * @param radius - the radius tried
 * @returns each candidate: its centre, its foot on the side measured from, its touch on the other
 * @see DERIV-fillet-beside-curve
 */
function candidates(
  sides: readonly [FilletSide, FilletSide],
  radius: number,
): readonly FilletCandidate[] {
  const [measured, running] = sides;

  return crossings(
    (t) => distanceFromSide(measured, filletCentre(running, radius, t), measured.limit) < radius,
    running.limit,
  ).map((t) => {
    const centre = filletCentre(running, radius, t);

    return { centre, measured: footOnSide(measured, centre, measured.limit), running: t };
  });
}

/**
 * Where a fillet of a radius touches a corner's two sides: the first candidate whose circle is
 * clear of both sides (`isValidTangency`). The centre runs along a curved side and is measured from
 * a straight one, whose distance needs no search (ADR-0047, `DERIV-fillet-beside-curve`).
 *
 * @kind geometry
 * @param sides - the side before, then the side after, oriented
 * @param radius - the radius tried
 * @returns its centre and its two tangent points' parameters, or `undefined` when none is drawn
 * @see DERIV-fillet-beside-curve
 */
export function filletTangency(
  sides: readonly [FilletSide, FilletSide],
  radius: number,
): FilletTangency | undefined {
  const [before, after] = sides;
  const isSwapped = !before.isStraight && after.isStraight;
  const ordered = isSwapped ? ([after, before] as const) : sides;
  const found = candidates(ordered, radius).find((candidate) =>
    isValidTangency(ordered, radius, candidate),
  );

  if (found === undefined) {
    return undefined;
  }

  const { centre, measured, running } = found;

  return isSwapped
    ? { after: measured, before: running, centre }
    : { after: running, before: measured, centre };
}
