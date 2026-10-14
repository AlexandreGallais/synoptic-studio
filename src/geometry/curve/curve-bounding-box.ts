import { cyclicItem } from "../contour";
import { boundingBox, cubicExtremes } from "../placement";

import { cubicSide } from "./cubic-side";
import { isCurvedSide } from "./is-curved-side";

import type { CurveCorner } from "../handles";
import type { BoundingBox } from "../placement";

/**
 * Box of a shape's curve: its points and the extremes of its curved sides, fillets left out (they
 * stay inside the corners' box, `DERIV-arc-bounding-box`). It is the shape's box (ADR-0046,
 * Product Owner, 2026-10-13: a square whose top bulges is as tall as its bulge).
 *
 * @kind geometry
 * @param corners - points and handles, in contour order
 * @returns the smallest upright rectangle around the curve
 * @see DERIV-cubic-sides
 */
export function curveBoundingBox(corners: readonly CurveCorner[]): BoundingBox {
  const extremes = corners.flatMap((corner, index) => {
    const next = cyclicItem(corners, index + 1, corner);

    return isCurvedSide(corner, next) ? cubicExtremes(cubicSide(corner, next)) : [];
  });

  return boundingBox([...corners.map((corner) => corner.point), ...extremes]);
}
