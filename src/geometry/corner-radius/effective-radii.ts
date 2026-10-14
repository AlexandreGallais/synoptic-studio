import { clampedRadii } from "./clamped-radii";
import { cornerTriples } from "./corner-triples";
import { turningAngles } from "./turning-angles";

import type { Corner } from "../contour";

/**
 * Effective radius of every corner: requested radii reduced locally and proportionally so that
 * the arcs of an edge never overlap (ADR-0007), the turning angles computed from the corners.
 *
 * @kind geometry
 * @param corners - vertices with their requested radii, in drawing order
 * @returns one effective radius per corner, 0 <= r′ <= r
 * @see DERIV-local-radius-clamp
 */
export function effectiveRadii(corners: readonly Corner[]): readonly number[] {
  const triples = cornerTriples(corners.map((corner) => corner.point));

  return clampedRadii(corners, turningAngles(triples));
}
