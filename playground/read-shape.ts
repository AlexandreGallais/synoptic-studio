import {
  RECTANGLE_CORNER_COUNT,
  freeShapeCorners,
  freeShapeRadii,
  isRegularPolygonCornerCount,
  isValidFreeShape,
  isValidRectangle,
  isValidRegularPolygon,
  rectangleAsFreeShape,
  regularPolygonCorners,
} from "../src";

import { readAnchor } from "./read-anchor";
import { readNumber } from "./read-number";
import { selectElement } from "./select-element";
import { updateKeptRadii } from "./update-kept-radii";

import type { ShapeReading } from "./shape-reading";
import type { Rectangle, RegularPolygon } from "../src";

/** Message shown when a value of the rectangle is not an integer ≥ 0. */
const INVALID_RECTANGLE = "Width, height and radius must be integers ≥ 0.";

/** Message shown when a value of the polygon is not allowed (Q15, Q19). */
const INVALID_POLYGON =
  "Width, height and radius must be integers ≥ 0, and corners an integer from 3 to 12.";

/**
 * Reads the rectangle typed in the width, height and radius inputs, with the radius of each of
 * its four corners (ADR-0029).
 *
 * @kind procedure
 * @param document - page document
 * @returns the rectangle, possibly invalid (checked by `isValidRectangle`, then as a free shape)
 * @see docs/backlog/stories/E01-F01-US-003-round-rectangle-corners.md
 */
function readRectangle(document: Document): Rectangle {
  return {
    height: readNumber(document, "height"),
    radii: updateKeptRadii(document, RECTANGLE_CORNER_COUNT),
    width: readNumber(document, "width"),
  };
}

/**
 * Reads the regular polygon typed in the inputs, with the radius of each of its corners
 * (ADR-0029) and its anchor (F07).
 *
 * @kind procedure
 * @param document - page document
 * @returns the polygon, possibly invalid (checked by `isValidRegularPolygon`)
 * @see docs/backlog/stories/E01-F02-US-007-shape-selector-playground.md
 */
function readPolygon(document: Document): RegularPolygon {
  const corners = readNumber(document, "corners");

  return {
    anchor: readAnchor(document),
    corners,
    height: readNumber(document, "height"),
    // One radius per corner; none for a number of corners refused.
    radii: updateKeptRadii(document, isRegularPolygonCornerCount(corners) ? corners : 0),
    width: readNumber(document, "width"),
  };
}

/**
 * Reads the rectangle typed in the inputs as the free shape the rectangle tool makes (ADR-0032),
 * with its corners when it is valid.
 *
 * @kind procedure
 * @param document - page document
 * @returns the reading of the rectangle
 * @see docs/backlog/stories/E01-F09-US-017-rectangle-as-free-shape.md
 */
function readRectangleShape(document: Document): ShapeReading {
  const rectangle = readRectangle(document);
  // The rectangle tool makes a free shape (ADR-0032, ADR-0033): its corners, width and height.
  const model = rectangleAsFreeShape(rectangle);
  // The rectangle tool takes integers (Q15); the free shape it makes is then checked too.
  const isValid = isValidRectangle(rectangle) && isValidFreeShape(model);

  return {
    corners: isValid ? freeShapeCorners(model) : [],
    isValid,
    message: INVALID_RECTANGLE,
    model,
    radii: freeShapeRadii(model),
  };
}

/**
 * Reads the regular polygon typed in the inputs, with its corners when it is valid.
 *
 * @kind procedure
 * @param document - page document
 * @returns the reading of the polygon
 * @see docs/backlog/stories/E01-F02-US-007-shape-selector-playground.md
 */
function readPolygonShape(document: Document): ShapeReading {
  const model = readPolygon(document);
  const isValid = isValidRegularPolygon(model);

  return {
    corners: isValid ? regularPolygonCorners(model) : [],
    isValid,
    message: INVALID_POLYGON,
    model,
    radii: model.radii,
  };
}

/**
 * Reads the shape chosen in the selector.
 *
 * @kind procedure
 * @param document - page document
 * @returns `polygon` or `rectangle`
 * @see docs/backlog/stories/E01-F02-US-007-shape-selector-playground.md
 */
function readShapeKind(document: Document): string {
  const select = selectElement(document, "shape");

  return select instanceof HTMLSelectElement && select.value === "polygon"
    ? "polygon"
    : "rectangle";
}

/**
 * Reads the shape chosen in the selector from the inputs, with its corners when it is valid.
 *
 * @kind procedure
 * @param document - page document
 * @returns the reading of the rectangle or of the polygon
 * @see docs/backlog/stories/E01-F02-US-007-shape-selector-playground.md
 */
export function readShape(document: Document): ShapeReading {
  return readShapeKind(document) === "polygon"
    ? readPolygonShape(document)
    : readRectangleShape(document);
}
