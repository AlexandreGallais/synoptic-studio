import { selectElement } from "../page";

import type { Point } from "../../src";

/** The zero vector: fields counted from the point itself. */
const ORIGIN = { x: 0, y: 0 };

/**
 * Where the handle fields are counted from, by the way chosen to type them: the page's origin
 * for x and y on the page, so that a field shows the handle's end (point + handle); the point
 * itself for dx and dy, so that a field shows the handle's vector (US-022, US-023).
 *
 * @kind procedure
 * @param document - page document
 * @param point - the point of the handles, as drawn
 * @returns what the fields' values are measured from, as a shift added to the handle
 * @see docs/backlog/stories/E14-F12-US-023-handles-relative-to-point.md
 */
export function readFieldOrigin(document: Document, point: Point): Point {
  const method = selectElement(document, "handle-method");

  return method instanceof HTMLSelectElement && method.value === "point" ? ORIGIN : point;
}
