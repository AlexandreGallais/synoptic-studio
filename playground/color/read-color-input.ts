import { selectElement } from "../page";

/** The story whose page holds the color fields. */
const STORY = "docs/backlog/stories/E15-F16-US-035-pick-a-color.md";

/**
 * Reads a field of a color: its hexadecimal or its opacity, an input of the page.
 *
 * @kind procedure
 * @param document - page document
 * @param id - its `id` attribute, without `#`
 * @returns the input element itself
 * @throws {TypeError} when the page's element is no input
 * @see docs/backlog/stories/E15-F16-US-035-pick-a-color.md
 */
export function readColorInput(document: Document, id: string): HTMLInputElement {
  const input = selectElement(document, id);

  if (!(input instanceof HTMLInputElement)) {
    throw new TypeError(`Playground page whose #${id} is no input (${STORY}).`);
  }

  return input;
}
