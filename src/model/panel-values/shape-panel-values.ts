import { boundingBox, contourBoundingBox, curveContour } from "../../geometry";
import { shapeCorners } from "../shape";

import type { PanelValues } from "./panel-values";
import type { Shape } from "../shape";

/**
 * The position and size the panel shows for a shape a tool makes (F38): the x and y of its
 * straight box, the smallest upright box around its contour as drawn, turned, its corners rounded
 * and its sides curved (`shapeCorners`, `curveContour`, `contourBoundingBox`), as for one shape
 * (ADR-0035), or, a shape 0 × 0 having a contour of no piece, the box of its corners, the one
 * point they lie on (`boundingBox`); its own width and height.
 *
 * @kind domain
 * @param shape - a valid shape of the model
 * @returns its values
 * @see docs/domain/interaction.md#_2-selector
 */
export function shapePanelValues(shape: Shape): PanelValues {
  const corners = shapeCorners(shape);
  const pieces = curveContour(corners);
  const box =
    pieces.length > 0 ? contourBoundingBox(pieces) : boundingBox(corners.map(({ point }) => point));

  return { height: shape.height, width: shape.width, x: box.minX, y: box.minY };
}
