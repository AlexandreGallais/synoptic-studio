import { selectElement } from "./select-element";

/**
 * Reads the number typed in an input, `NaN` when it is empty or not a number.
 *
 * @kind procedure
 * @param document - page document
 * @param id - id of the input
 * @returns the typed value
 * @see docs/backlog/stories/E01-F01-US-002-sharp-rectangle-in-playground.md
 */
export function readNumber(document: Document, id: string): number {
  const input = selectElement(document, id);

  return input instanceof HTMLInputElement ? input.valueAsNumber : NaN;
}
