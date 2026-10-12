import { cyclicItem } from "./cyclic-item";
import { effectiveRadii } from "./effective-radii";
import { filletArc } from "./fillet-arc";
import { hasLength } from "./has-length";

import type { Arc } from "./arc";
import type { ContourPiece } from "./contour-piece";
import type { Corner } from "./corner";

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
  const points = corners.map((corner) => corner.point);
  const radii = effectiveRadii(corners);
  const arcs = points.map((vertex, index) =>
    filletArc(
      {
        next: cyclicItem(points, index + 1, vertex),
        previous: cyclicItem(points, index - 1, vertex),
        vertex,
      },
      cyclicItem(radii, index, 0),
    ),
  );

  return arcs
    .flatMap((arc, index): ContourPiece[] => {
      const next: Arc = cyclicItem(arcs, index + 1, arc);

      return [{ end: next.start, kind: "segment", start: arc.end }, next];
    })
    .filter((piece) => hasLength(piece));
}
