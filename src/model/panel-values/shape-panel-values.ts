import { shapeStraightBox } from "../../geometry";
import { shapeCorners } from "../shape";

import type { PanelValues } from "./panel-values";
import type { Shape } from "../shape";

/**
 * The position and size the panel shows for a shape a tool makes (F38): the x and y of its
 * straight box, the smallest upright box around its contour as drawn, turned, its corners rounded
 * and its sides curved, or the one point a shape 0 × 0 lies on (`shapeCorners`,
 * `shapeStraightBox`), as for one shape (ADR-0035); its own width and height.
 *
 * @kind domain
 * @param shape - a valid shape of the model
 * @returns its values
 * @see docs/domain/interaction.md#_2-selector
 */
export function shapePanelValues(shape: Shape): PanelValues {
  const box = shapeStraightBox(shapeCorners(shape));

  return { height: shape.height, width: shape.width, x: box.minX, y: box.minY };
}
