import { selectElement } from "../page";

/** The fields of a text's position, X then Y. */
const POSITION_FIELDS = ["shape-x", "shape-y"];

/**
 * Reads what the X and Y fields show, as typed or written.
 *
 * @kind procedure
 * @param document - page document
 * @returns the two values, X first
 * @see docs/backlog/stories/E15-F05-US-038-a-turned-text-grows-in-place.md
 */
export function readPositionFields(document: Document): readonly string[] {
  return POSITION_FIELDS.map((id) => {
    const field = selectElement(document, id);

    return field instanceof HTMLInputElement ? field.value : "";
  });
}
