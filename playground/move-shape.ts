import { readPosition } from "./read-position";
import { readStoredFreeShape } from "./read-stored-free-shape";
import { selectElement } from "./select-element";
import { writeStoredFreeShape } from "./write-stored-free-shape";

import type { Vector } from "../src";

/**
 * Moves the shape of the page: its position, the top-left of its box before it turns, goes by a
 * vector (ADR-0035); a kept free shape moves its own position, the rectangle and the polygon the
 * one the page keeps.
 *
 * @kind procedure
 * @param document - page document
 * @param delta - move, in units of the SVG
 * @see docs/backlog/stories/E01-F04-US-018-turn-a-shape.md
 */
export function moveShape(document: Document, delta: Vector): void {
  const kept = readStoredFreeShape(document);

  if (kept !== undefined) {
    writeStoredFreeShape(document, { ...kept, x: kept.x + delta.x, y: kept.y + delta.y });

    return;
  }

  const position = readPosition(document);
  const { dataset } = selectElement(document, "shape");

  dataset["x"] = String(position.x + delta.x);
  dataset["y"] = String(position.y + delta.y);
}
