import {
  RECTANGLE_CORNER_COUNT,
  curveCornerOf,
  freeShapeCorners,
  freeShapeRadii,
  isRegularPolygonCornerCount,
  isValidFreeShape,
  isValidRectangle,
  isValidRegularPolygon,
  rectangleAsFreeShape,
  regularPolygonCorners,
  turnedCorners,
} from "../../src";
import { readNumber, readShapeKind } from "../page";
import { readAngle } from "../position";

import { readAnchor } from "./read-anchor";
import { readFreeShapeShape } from "./read-free-shape-shape";
import { readPosition } from "./read-position";
import { readStoredFreeShape } from "./read-stored-free-shape";
import { updateKeptRadii } from "./update-kept-radii";

import type { ShapeReading } from "./shape-reading";
import type { Rectangle, RegularPolygon } from "../../src";

/** Message shown when a value of the rectangle is not an integer ≥ 0. */
const INVALID_RECTANGLE =
  "Width, height and radius must be integers ≥ 0, and the angle an integer.";

/** Message shown when a value of the polygon is not allowed (Q15, Q19). */
const INVALID_POLYGON =
  "Width, height and radius must be integers ≥ 0, corners an integer from 3 to 12, and the angle an integer.";

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
  // The tool's position is a creation value; the page moves the free shape it makes (ADR-0035).
  return {
    angle: readAngle(document),
    height: readNumber(document, "height"),
    radii: updateKeptRadii(document, RECTANGLE_CORNER_COUNT),
    width: readNumber(document, "width"),
    x: 0,
    y: 0,
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
  const position = readPosition(document);

  return {
    anchor: readAnchor(document),
    angle: readAngle(document),
    corners,
    height: readNumber(document, "height"),
    // One radius per corner; none for a number of corners refused.
    radii: updateKeptRadii(document, isRegularPolygonCornerCount(corners) ? corners : 0),
    stretched: document.querySelector<HTMLInputElement>("#stretched")?.checked === true,
    width: readNumber(document, "width"),
    ...position,
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
  const model = { ...rectangleAsFreeShape(rectangle), ...readPosition(document) };
  // The rectangle tool takes integers (Q15); the free shape it makes is then checked too.
  const isValid = isValidRectangle(rectangle) && isValidFreeShape(model);

  return {
    corners: isValid ? turnedCorners(freeShapeCorners(model), model) : [],
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
    corners: isValid
      ? turnedCorners(
          regularPolygonCorners(model).map((corner) => curveCornerOf(corner)),
          model,
        )
      : [],
    isValid,
    message: INVALID_POLYGON,
    model,
    radii: model.radii,
  };
}

/**
 * Reads the shape chosen in the selector from the inputs, with its corners when it is valid.
 *
 * @kind procedure
 * @param document - page document
 * @returns the reading of the rectangle, the polygon or the free shape kept (F08)
 * @see docs/backlog/stories/E01-F02-US-007-shape-selector-playground.md
 */
export function readShape(document: Document): ShapeReading {
  const kept = readStoredFreeShape(document);

  if (kept !== undefined) {
    return readFreeShapeShape(document, kept);
  }

  return readShapeKind(document) === "polygon"
    ? readPolygonShape(document)
    : readRectangleShape(document);
}
