import { readTypingWay } from "./read-typing-way";

import type { Point } from "../../src";

/** The zero vector: fields that show the handle's vector itself. */
const NO_SHIFT = { x: 0, y: 0 };

/**
 * What the handle fields add to a handle's vector, by the way chosen to type them: the point, for
 * x and y on the page, so that a field shows the handle's end; nothing, for dx and dy, so that a
 * field shows the vector from the point (US-022, US-023).
 *
 * @kind procedure
 * @param document - page document
 * @param point - the point of the handles, as drawn
 * @returns the shift: the point, or the zero vector
 * @see docs/backlog/stories/E14-F12-US-023-handles-relative-to-point.md
 */
export function readFieldShift(document: Document, point: Point): Point {
  return readTypingWay(document) === "point" ? NO_SHIFT : point;
}
