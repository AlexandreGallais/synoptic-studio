import { cyclicItem } from "../contour";

import { clampedRadii } from "./clamped-radii";
import { cornerTriples } from "./corner-triples";
import { filletArc } from "./fillet-arc";
import { turningAngles } from "./turning-angles";

import type { Arc, Corner } from "../contour";

/**
 * The fillet arc of each corner, its radius clamped (`DERIV-local-radius-clamp`), each corner's
 * triple and turning angle computed once (F11.AC3); a corner of radius 0 gets an arc without
 * length at its point.
 *
 * @kind geometry
 * @param corners - vertices with their requested radii, clockwise from the top-left vertex (Q11)
 * @returns one arc per corner, in the same order
 * @see DERIV-fillet-arc
 */
export function filletArcs(corners: readonly Corner[]): readonly Arc[] {
  const triples = cornerTriples(corners.map((corner) => corner.point));
  const angles = turningAngles(triples);
  const radii = clampedRadii(corners, angles);

  return triples.map((triple, index) =>
    filletArc(triple, cyclicItem(angles, index, 0), cyclicItem(radii, index, 0)),
  );
}
