import { selectElement } from "../page";

/**
 * Reads the value of a text field, a text area or a choice of the page.
 *
 * @kind procedure
 * @param document - page document
 * @param id - its `id` attribute, without `#`
 * @returns its value; empty for an element that holds none
 * @see docs/backlog/stories/E15-F05-US-031-place-a-text.md
 */
export function readFieldValue(document: Document, id: string): string {
  const field = selectElement(document, id);

  return "value" in field && typeof field.value === "string" ? field.value : "";
}
