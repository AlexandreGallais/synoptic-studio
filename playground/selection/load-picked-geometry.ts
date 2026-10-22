import { NO_VERTEX_SELECTION, freeShapeRadii, isFreeShape, isRegularPolygon } from "../../src";
import { writeAnchorValue, writeStretchedValue } from "../guided-test";
import { selectElement, writeEditedKind, writeInputValue, writeSelection } from "../page";
import { writeStoredFreeShape } from "../shape";

import type { RegularPolygon, Shape } from "../../src";

/**
 * Writes the radius of each vertex of a shape picked on the radius field (`data-radii`): the
 * redraw that follows a pick shows in the field their common radius, or Mixed when they differ
 * (`writeRadiusField`).
 *
 * @kind procedure
 * @param document - page document
 * @param radii - the radius of each vertex, in contour order
 * @see docs/domain/interaction.md#_2-selector
 */
function writePickedRadii(document: Document, radii: readonly number[]): void {
  selectElement(document, "radius").dataset["radii"] = radii.join(" ");
}

/**
 * Writes the settings of a polygon picked: its corners, its stretched option, its anchor and its
 * radii.
 *
 * @kind procedure
 * @param document - page document
 * @param polygon - a regular polygon of the drawing, clicked
 * @see docs/domain/interaction.md#_2-selector
 */
function writePickedPolygon(document: Document, polygon: RegularPolygon): void {
  writeInputValue(document, "corners", polygon.corners);
  writeStretchedValue(document, polygon.stretched);
  writeAnchorValue(document, `${polygon.anchor.horizontal} ${polygon.anchor.vertical}`);
  writePickedRadii(document, polygon.radii);
}

/**
 * Writes the settings of a shape picked by its tool's: a polygon's (`writePickedPolygon`), a
 * rectangle's radii, stored as a free shape (ADR-0032); a round shape has none.
 *
 * @kind procedure
 * @param document - page document
 * @param shape - a rectangle stored as a free shape, a polygon or a round shape, clicked
 * @see docs/domain/interaction.md#_2-selector
 */
function writePickedSettings(document: Document, shape: Shape): void {
  if (isRegularPolygon(shape)) {
    writePickedPolygon(document, shape);

    return;
  }

  if (isFreeShape(shape)) {
    writePickedRadii(document, freeShapeRadii(shape));
  }
}

/**
 * Loads a rectangle, a polygon or a round shape picked into the panel, its kind chosen: its width,
 * its height and the top-left of its box before it turns (ADR-0035), kept on the page, then its
 * settings (`writePickedSettings`).
 *
 * @kind procedure
 * @param document - page document
 * @param kind - `rectangle`, `polygon` or `round`
 * @param shape - a rectangle stored as a free shape, a polygon or a round shape, clicked
 * @see docs/domain/interaction.md#_2-selector
 */
function loadPickedTool(document: Document, kind: string, shape: Shape): void {
  const { dataset } = selectElement(document, "shape");

  writeEditedKind(document, kind);
  writeInputValue(document, "width", shape.width);
  writeInputValue(document, "height", shape.height);
  dataset["x"] = String(shape.x);
  dataset["y"] = String(shape.y);
  writePickedSettings(document, shape);
}

/**
 * Loads a shape picked that is not a text into the panel, without firing any event (F36): its
 * angle, then a free shape kept as one (`writeStoredFreeShape`), no point selected, or a rectangle
 * as a Rectangle, though stored as a free shape (ADR-0032), a polygon or a round shape as its
 * tool shows it (`loadPickedTool`).
 *
 * @kind procedure
 * @param document - page document
 * @param kind - the kind the panel showed it as: `rectangle`, `polygon`, `round` or `free`
 * @param shape - a shape of the drawing, clicked
 * @see docs/domain/interaction.md#_2-selector
 */
export function loadPickedGeometry(document: Document, kind: string, shape: Shape): void {
  writeInputValue(document, "angle", shape.angle);

  if (kind === "free" && isFreeShape(shape)) {
    writeStoredFreeShape(document, shape);
    writeSelection(document, NO_VERTEX_SELECTION);

    return;
  }

  loadPickedTool(document, kind, shape);
}
