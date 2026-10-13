import { resizeRadii } from "../../src";
import { readNumber, selectElement } from "../page";

/**
 * Reads the requested radius of each vertex. When the vertices were given different radii, the
 * page keeps them on the radius input (`data-radii`): they are resized by vertex number to the
 * number of vertices and kept so, even while the shape is refused, so that a removed vertex
 * never gets its radius back (F03.AC3). Otherwise the radius typed goes to every vertex.
 *
 * @kind procedure
 * @param document - page document
 * @param count - number of vertices of the shape, 0 when it is refused
 * @returns `count` radii, possibly invalid (checked with the shape)
 * @see docs/backlog/stories/E01-F03-US-011-radius-field-mixed.md
 */
export function updateKeptRadii(document: Document, count: number): readonly number[] {
  const field = selectElement(document, "radius");
  const kept = field.dataset["radii"];

  if (kept === undefined) {
    const radius = readNumber(document, "radius");

    return Array.from({ length: count }, () => radius);
  }

  const radii = resizeRadii(kept.split(" ").map(Number), count);

  // No number of vertices (corners refused): keep the radii as they are.
  if (count > 0) {
    field.dataset["radii"] = radii.join(" ");
  }

  return radii;
}
