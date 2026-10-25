import { selectElement } from "./select-element";

/**
 * Reads the shape chosen in the selector.
 *
 * @kind procedure
 * @param document - page document
 * @returns `rectangle`, `polygon`, `round`, `free` or `text`
 */
export function readShapeKind(document: Document): string {
  const select = selectElement(document, "shape");

  return select instanceof HTMLSelectElement ? select.value : "rectangle";
}
