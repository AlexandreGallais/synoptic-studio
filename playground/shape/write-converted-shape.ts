import {
  hasArea,
  isFreeShape,
  isRoundShape,
  regularPolygonAsFreeShape,
  roundShapeAsFreeShape,
} from "../../src";

import { readShape } from "./read-shape";
import { writeStoredFreeShape } from "./write-stored-free-shape";

import type { FreeShape, RegularPolygon, RoundShape } from "../../src";

/**
 * Reads the regular polygon or the round shape shown, when it is valid.
 *
 * @kind procedure
 * @param document - page document
 * @returns that shape, `undefined` for a free shape or a refused one
 */
function readConvertibleShape(document: Document): RegularPolygon | RoundShape | undefined {
  const { isValid, model } = readShape(document);

  return isValid && !isFreeShape(model) ? model : undefined;
}

/**
 * The free shape a regular polygon or a round shape converts to, unmoved: the polygon's corners,
 * or the round shape's 4 points with their handles (F10.AC4).
 *
 * @kind procedure
 * @param shape - valid shape of width and height > 0
 * @returns the free shape drawing it
 */
function readFreeShapeOf(shape: RegularPolygon | RoundShape): FreeShape {
  return isRoundShape(shape) ? roundShapeAsFreeShape(shape) : regularPolygonAsFreeShape(shape);
}

/**
 * Converts the valid regular polygon or round shape shown into a free shape, unmoved, and keeps
 * it on the page: a round shape becomes its 4 points with their handles (F10.AC4). A shape of
 * width or height 0 has no box to stretch and is not converted (F08.AC1, ADR-0033).
 *
 * @kind procedure
 * @param document - page document
 */
export function writeConvertedShape(document: Document): void {
  const shape = readConvertibleShape(document);

  if (shape !== undefined && hasArea(shape)) {
    writeStoredFreeShape(document, readFreeShapeOf(shape));
  }
}
