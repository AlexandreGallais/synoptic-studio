import { readShapePosition } from "./read-shape-position";
import { writeShapePosition } from "./write-shape-position";

import type { Vector } from "../../src";

/**
 * Moves the shape of the page: its position, the top-left of its box before it turns, goes by a
 * vector (ADR-0035).
 *
 * @kind procedure
 * @param document - page document
 * @param delta - move, in units of the SVG
 * @see docs/backlog/stories/E01-F04-US-018-turn-a-shape.md
 */
export function moveShape(document: Document, delta: Vector): void {
  const position = readShapePosition(document);

  writeShapePosition(document, { x: position.x + delta.x, y: position.y + delta.y });
}
