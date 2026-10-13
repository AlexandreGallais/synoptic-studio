import { regularPolygonAsFreeShape } from "../../src";

import { readShape } from "./read-shape";
import { writeStoredFreeShape } from "./write-stored-free-shape";

import type { RegularPolygon } from "../../src";

/**
 * Reads the regular polygon shown, when it is valid.
 *
 * @kind procedure
 * @param document - page document
 * @returns the polygon, `undefined` for another shape or a refused one
 * @see docs/backlog/stories/E01-F08-US-014-convert-and-move.md
 */
function readValidPolygon(document: Document): RegularPolygon | undefined {
  const { isValid, model } = readShape(document);

  return isValid && "anchor" in model ? model : undefined;
}

/**
 * Converts the valid regular polygon shown into a free shape, unmoved, and keeps it on the page;
 * a polygon of width or height 0 has no box to stretch and is not converted (F08.AC1, ADR-0033).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E01-F08-US-014-convert-and-move.md
 */
export function writeConvertedPolygon(document: Document): void {
  const polygon = readValidPolygon(document);

  if (polygon !== undefined && polygon.width * polygon.height > 0) {
    writeStoredFreeShape(document, regularPolygonAsFreeShape(polygon));
  }
}
