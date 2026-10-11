import { resizeRadii } from "../src";

import { readNumber } from "./read-number";
import { selectElement } from "./select-element";

/**
 * Reads the requested radius of each vertex. When the vertices were given different radii, the
 * page keeps them on the radius input (`data-radii`), resized by vertex number to the number of
 * vertices (F03.AC3); otherwise the radius typed goes to every vertex.
 *
 * @kind procedure
 * @param document - page document
 * @param count - number of vertices of the shape, 0 when it is refused
 * @returns `count` radii, possibly invalid (checked with the shape)
 * @see docs/backlog/stories/E01-F03-US-011-radius-field-mixed.md
 */
export function readRadii(document: Document, count: number): readonly number[] {
  const kept = selectElement(document, "radius").dataset["radii"];

  if (kept === undefined) {
    const radius = readNumber(document, "radius");

    return Array.from({ length: count }, () => radius);
  }

  return resizeRadii(kept.split(" ").map(Number), count);
}
