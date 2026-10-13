import { selectInput } from "./select-input";

/**
 * Types a value into an input, as a user would: its `input` event.
 *
 * @kind procedure
 * @param id - input id
 * @param value - typed text
 * @see docs/backlog/stories/E14-F11-EN-021-shared-test-helpers.md
 */
export function typeValue(id: string, value: string): void {
  const input = selectInput(id);

  input.value = value;
  input.dispatchEvent(new Event("input", { bubbles: true }));
}
