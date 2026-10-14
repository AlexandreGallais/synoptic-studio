import { boxSize } from "../layout-box";
import { boundingBox } from "../placement";

import type { Point } from "../../math";

/**
 * Aspect of a set of points: the height of their bounding box over its width
 * (`DERIV-stretched-polygon` step 4).
 *
 * @kind geometry
 * @param points - points spanning the x axis
 * @returns height over width
 * @see DERIV-stretched-polygon
 */
export function boxAspect(points: readonly Point[]): number {
  const { height, width } = boxSize(boundingBox(points));

  return height / width;
}
