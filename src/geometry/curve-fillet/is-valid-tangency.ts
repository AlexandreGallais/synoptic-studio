import { EPSILON } from "../../math";
import { distanceFromSide } from "../fillet-side";

import { isClearOfSide } from "./is-clear-of-side";

import type { Point } from "../../math";
import type { FilletSide } from "../fillet-side";

/** A candidate fillet: its centre and its feet on the side measured from and the side run along. */
export type FilletCandidate = {
  /** Its centre. */
  readonly centre: Point;
  /** Parameter of its foot on the side measured from. */
  readonly measured: number;
  /** Parameter where it touches the side run along. */
  readonly running: number;
};

/**
 * Whether a candidate fillet is drawn: its centre the radius from the side it was measured from,
 * its foot there before the side's limit, its circle clear of both whole sides (ADR-0047,
 * `DERIV-fillet-beside-curve` step 11).
 *
 * @kind geometry
 * @param sides - the side measured from, then the side run along, oriented
 * @param radius - the radius tried
 * @param candidate - the candidate's centre and parameters
 * @returns `true` when the arc can be drawn there
 * @see DERIV-fillet-beside-curve
 */
export function isValidTangency(
  sides: readonly [FilletSide, FilletSide],
  radius: number,
  candidate: FilletCandidate,
): boolean {
  const [measured, running] = sides;
  const { centre } = candidate;
  const gap = distanceFromSide(measured, centre, measured.limit) - radius;

  return (
    Math.abs(gap) <= EPSILON * Math.max(1, radius) &&
    candidate.measured < measured.limit &&
    isClearOfSide(measured, centre, radius) &&
    isClearOfSide(running, centre, radius)
  );
}
