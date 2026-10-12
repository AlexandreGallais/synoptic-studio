import { selectElement } from "./select-element";

/**
 * Reads the shape chosen in the selector.
 *
 * @kind procedure
 * @param document - page document
 * @returns `rectangle`, `polygon` or `free`
 * @see docs/backlog/stories/E01-F02-US-007-shape-selector-playground.md
 */
export function readShapeKind(document: Document): string {
  const select = selectElement(document, "shape");

  return select instanceof HTMLSelectElement ? select.value : "rectangle";
}
