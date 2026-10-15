import { readDrawnText } from "./read-drawn-text";
import { readPositionFields } from "./read-position-fields";
import { readTypedCoordinate } from "./read-typed-coordinate";

import type { Point } from "../../src";

/**
 * Reads the text's position from the X and Y fields (US-038): while they still show the position
 * of the text last drawn, at 5 decimals, its exact position, a fraction once a turned text has
 * grown; otherwise the integers typed (`readTypedCoordinate`), a fraction typed refused.
 *
 * @kind procedure
 * @param document - page document
 * @returns the top-left of its block, or its box, before it turns
 * @see docs/backlog/stories/E15-F05-US-038-a-turned-text-grows-in-place.md
 */
export function readTextPosition(document: Document): Point {
  const drawn = readDrawnText(document);
  const fields = readPositionFields(document);
  const isShown = fields.every((value, index) => value === drawn?.fields[index]);

  return drawn !== undefined && isShown
    ? { x: drawn.text.x, y: drawn.text.y }
    : { x: readTypedCoordinate(document, "shape-x"), y: readTypedCoordinate(document, "shape-y") };
}
