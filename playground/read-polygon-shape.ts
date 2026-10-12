import {
  isRegularPolygonCornerCount,
  isValidRegularPolygon,
  regularPolygonCorners,
  turnedCorners,
} from "../src";

import { readAnchor } from "./read-anchor";
import { readAngle } from "./read-angle";
import { readNumber } from "./read-number";
import { readPosition } from "./read-position";
import { updateKeptRadii } from "./update-kept-radii";

import type { ShapeReading } from "./shape-reading";
import type { RegularPolygon } from "../src";

/** Message shown when a value of the polygon is not allowed (Q15, Q19). */
const INVALID_POLYGON =
  "Width, height and radius must be integers ≥ 0, corners an integer from 3 to 12, and the angle an integer.";

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
    width: readNumber(document, "width"),
    ...position,
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
export function readPolygonShape(document: Document): ShapeReading {
  const model = readPolygon(document);
  const isValid = isValidRegularPolygon(model);

  return {
    corners: isValid ? turnedCorners(regularPolygonCorners(model), model) : [],
    isValid,
    message: INVALID_POLYGON,
    model,
    radii: model.radii,
  };
}
