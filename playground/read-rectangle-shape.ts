import {
  RECTANGLE_CORNER_COUNT,
  freeShapeCorners,
  freeShapeRadii,
  isValidFreeShape,
  isValidRectangle,
  rectangleAsFreeShape,
  turnedCorners,
} from "../src";

import { readAngle } from "./read-angle";
import { readNumber } from "./read-number";
import { readPosition } from "./read-position";
import { updateKeptRadii } from "./update-kept-radii";

import type { ShapeReading } from "./shape-reading";
import type { Rectangle } from "../src";

/** Message shown when a value of the rectangle is not an integer ≥ 0. */
const INVALID_RECTANGLE =
  "Width, height and radius must be integers ≥ 0, and the angle an integer.";

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
 * Reads the rectangle typed in the inputs as the free shape the rectangle tool makes (ADR-0032),
 * with its corners when it is valid.
 *
 * @kind procedure
 * @param document - page document
 * @returns the reading of the rectangle
 * @see docs/backlog/stories/E01-F09-US-017-rectangle-as-free-shape.md
 */
export function readRectangleShape(document: Document): ShapeReading {
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
