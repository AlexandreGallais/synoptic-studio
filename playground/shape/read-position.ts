import { selectElement } from "../page";

import type { Point } from "../../src";

/**
 * Reads the position of the rectangle or the polygon the page keeps: the top-left of its box
 * before it turns (ADR-0035), (0, 0) until it is moved. A kept free shape holds its own.
 *
 * @kind procedure
 * @param document - page document
 * @returns the position, in the coordinates of the SVG
 * @see docs/backlog/stories/E01-F04-US-018-turn-a-shape.md
 */
export function readPosition(document: Document): Point {
  const { x = "0", y = "0" } = selectElement(document, "shape").dataset;

  return { x: Number(x), y: Number(y) };
}
