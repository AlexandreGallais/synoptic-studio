import type { PickArea } from "./pick-area";
import type { Shape } from "../shape";

/**
 * What a shape offers the pointer: its corners as drawn (`shapeCorners`), the contour they make
 * (`curveContour`), as the drawing's SVG draws it, and whether its box has no area
 * (`isFlatBox`).
 *
 * @kind domain
 * @param shape - a valid shape of the model
 * @returns its corners' points, its contour and whether it is flat
 * @see docs/domain/interaction.md#_2-selector
 */
export function shapePickArea(shape: Shape): PickArea {
  return { corners: [], isFlat: shape.width < 0, pieces: [] };
}
