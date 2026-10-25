import { selectElement } from "../page";

/** The story whose page holds the color fields. */
const STORY = "US-035";

/**
 * Reads a field of a color: its hexadecimal or its opacity, an input of the page.
 *
 * @kind procedure
 * @param document - page document
 * @param id - its `id` attribute, without `#`
 * @returns the input element itself
 * @throws {TypeError} when the page's element is no input
 */
export function readColorInput(document: Document, id: string): HTMLInputElement {
  const input = selectElement(document, id);

  if (!(input instanceof HTMLInputElement)) {
    throw new TypeError(`Playground page whose #${id} is no input (${STORY}).`);
  }

  return input;
}
