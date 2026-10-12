import { readFreeShapeShape } from "./read-free-shape-shape";
import { readPolygonShape } from "./read-polygon-shape";
import { readRectangleShape } from "./read-rectangle-shape";
import { readRoundShape } from "./read-round-shape";
import { readShapeKind } from "./read-shape-kind";
import { readStoredFreeShape } from "./read-stored-free-shape";

import type { ShapeReading } from "./shape-reading";

/** Readers of the shapes chosen in the selector other than the rectangle (F02, F10). */
const SHAPE_READERS: Readonly<Record<string, (document: Document) => ShapeReading>> = {
  polygon: readPolygonShape,
  round: readRoundShape,
};

/**
 * Reads the shape chosen in the selector from the inputs, with its corners when it is valid.
 *
 * @kind procedure
 * @param document - page document
 * @returns the reading of the rectangle, the polygon or the free shape kept (F08)
 * @see docs/backlog/stories/E01-F02-US-007-shape-selector-playground.md
 */
export function readShape(document: Document): ShapeReading {
  const kept = readStoredFreeShape(document);

  if (kept !== undefined) {
    return readFreeShapeShape(document, kept);
  }

  const reader = SHAPE_READERS[readShapeKind(document)] ?? readRectangleShape;

  return reader(document);
}
