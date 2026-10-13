import { selectInput } from "./select-input";

/**
 * Value of an input or a select.
 *
 * @kind procedure
 * @param id - its id
 * @returns the value shown
 * @see docs/backlog/stories/E14-F11-EN-021-shared-test-helpers.md
 */
export function readValue(id: string): string {
  return selectInput(id).value;
}
