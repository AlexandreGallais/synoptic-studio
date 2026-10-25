import { writeShapeValues } from "../shape-fields";

/** The shape the page opens with, as `index.html` sets it. */
const DEFAULT_SHAPE = {
  anchor: "mid mid",
  corners: 6,
  height: 80,
  radius: 10,
  shape: "rectangle",
  width: 120,
} as const;

/**
 * Puts back the shape the page opens with, alone: a rectangle 120 × 80, radius 10, straight, at
 * (0, 0), no free shape kept, no point selected, the shapes placed and the settings each kind kept
 * forgotten, the Text tool's fields the page's.
 *
 * @kind procedure
 * @param document - page document
 */
export function writeDefaultShape(document: Document): void {
  writeShapeValues(document, DEFAULT_SHAPE);
}
