import { readNumber } from "../page";

import { readDrawnText } from "./read-drawn-text";
import { readPositionFields } from "./read-position-fields";

import type { Point } from "../../src";

/**
 * Reads the text's position from the X and Y fields (US-038): while they still show the position
 * of the text last drawn, at 5 decimals, its exact position; otherwise the numbers typed, a
 * fraction accepted.
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
    : { x: readNumber(document, "shape-x"), y: readNumber(document, "shape-y") };
}
