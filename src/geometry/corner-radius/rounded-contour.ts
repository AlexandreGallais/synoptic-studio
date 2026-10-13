import { cyclicItem } from "../contour";

import { clampedRadii } from "./clamped-radii";
import { cornerTriples } from "./corner-triples";
import { filletArc } from "./fillet-arc";
import { hasLength } from "./has-length";
import { turningAngles } from "./turning-angles";

import type { Arc, ContourPiece, Corner } from "../contour";

/**
 * Evaluated contour of a shape with corner radii: a closed sequence of segments and arcs.
 *
 * Radii are first clamped (`DERIV-local-radius-clamp`); each corner gets its fillet arc; each edge
 * becomes the segment between the arcs of its two corners. The sequence starts at the end of the
 * first corner's arc and closes with that arc (`DERIV-fillet-arc` step 7); pieces without length
 * are dropped. Vertices that turn back on themselves (spikes, τ = ±π) are outside the derivation
 * (Q16: such a spike is consumed by its fillet).
 *
 * @kind geometry
 * @param corners - vertices with their requested radii, clockwise from the top-left vertex (Q11)
 * @returns segments and arcs in drawing order, each starting where the previous one ends, within
 * `EPSILON`
 * @see DERIV-fillet-arc
 */
export function roundedContour(corners: readonly Corner[]): readonly ContourPiece[] {
  // Each corner's triple and turning angle, computed once and shared by the clamp and the fillets.
  const triples = cornerTriples(corners.map((corner) => corner.point));
  const angles = turningAngles(triples);
  const radii = clampedRadii(corners, angles);
  const arcs = triples.map((triple, index) =>
    filletArc(triple, cyclicItem(angles, index, 0), cyclicItem(radii, index, 0)),
  );

  return arcs
    .flatMap((arc, index): ContourPiece[] => {
      const next: Arc = cyclicItem(arcs, index + 1, arc);

      return [{ end: next.start, kind: "segment", start: arc.end }, next];
    })
    .filter((piece) => hasLength(piece));
}
