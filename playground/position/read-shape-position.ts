import { readPosition, readStoredFreeShape } from "../shape";

import type { Point } from "../../src";

/**
 * Reads the position of the shape of the page, the top-left of its box before it turns
 * (ADR-0035): a kept free shape's own, otherwise the one the page keeps for the rectangle and
 * the polygon.
 *
 * @kind procedure
 * @param document - page document
 * @returns the position, in the coordinates of the SVG
 * @see docs/backlog/stories/E01-F04-US-019-edit-a-turned-shape.md
 */
export function readShapePosition(document: Document): Point {
  const kept = readStoredFreeShape(document);

  return kept === undefined ? readPosition(document) : { x: kept.x, y: kept.y };
}
