import { norm, subtract } from "../math";

import { cyclicItem } from "./cyclic-item";

import type { Point } from "../math";

/**
 * Length of every edge of a cyclic contour: edge `i` goes from vertex `i` to vertex `i + 1`.
 *
 * @kind geometry
 * @param contour - vertices in drawing order
 * @returns one length per edge, the last edge closing the contour (Q11)
 * @see DERIV-local-radius-clamp
 */
export function edgeLengths(contour: readonly Point[]): readonly number[] {
  return contour.map((vertex, index) =>
    norm(subtract(cyclicItem(contour, index + 1, vertex), vertex)),
  );
}
