import {
  NO_HANDLES,
  cornersWithRadii,
  formatSvgNumber,
  isModelSize,
  isValidFreeShape,
  shapeCorners,
} from "../../src";
import { readShownText } from "../applied-values";
import { readNumber, selectElement } from "../page";
import { readAngle } from "../position";

import { updateKeptRadii } from "./update-kept-radii";

import type { ShapeReading } from "./shape-reading";
import type { FreeShape } from "../../src";

/** Message shown when a value of the free shape is not allowed. */
const INVALID_FREE_SHAPE =
  "Width, height and radius typed must be integers ≥ 0, the angle an integer, and the points must span some width and height.";

/**
 * Reads the text of an input as the page reads it, the value applied (`readShownText`).
 *
 * @kind procedure
 * @param document - page document
 * @param id - id of the input to read
 * @returns the characters in the input, empty when the element is no input
 */
function readInputText(document: Document, id: string): string {
  const input = selectElement(document, id);

  return input instanceof HTMLInputElement ? readShownText(input) : "";
}

/**
 * Reads a size of the free shape: the one it has while its input still shows it (fractional as a
 * conversion leaves it, Q27), otherwise the integer typed.
 *
 * @kind procedure
 * @param document - page document
 * @param id - `width` or `height`
 * @param kept - the size the free shape has
 * @returns the size, and whether it is allowed
 */
function readFreeSize(
  document: Document,
  id: string,
  kept: number,
): { readonly size: number; readonly isValid: boolean } {
  const isKept = readInputText(document, id) === formatSvgNumber(kept);
  const size = isKept ? kept : readNumber(document, id);

  return { isValid: isKept || isModelSize(size), size };
}

/**
 * Reads the free shape the page keeps, stretched to the width and height in the inputs and with
 * the radii of the radius field, with its corners when it is valid (ADR-0033).
 *
 * @kind procedure
 * @param document - page document
 * @param kept - the free shape the page keeps
 * @returns the reading of the free shape
 */
export function readFreeShapeShape(document: Document, kept: FreeShape): ShapeReading {
  const width = readFreeSize(document, "width", kept.width);
  const height = readFreeSize(document, "height", kept.height);
  const radii = updateKeptRadii(document, kept.corners.length);
  const points = kept.corners.map((corner) => corner.point);
  const model = {
    angle: readAngle(document),
    corners: cornersWithRadii(points, radii).map((corner, index) => ({
      ...corner,
      handles: kept.corners[index]?.handles ?? NO_HANDLES,
    })),
    height: height.size,
    width: width.size,
    x: kept.x,
    y: kept.y,
  };
  const isValid = [width.isValid, height.isValid, isValidFreeShape(model)].every(Boolean);

  return {
    corners: isValid ? shapeCorners(model) : [],
    isValid,
    message: INVALID_FREE_SHAPE,
    model,
    radii,
  };
}
