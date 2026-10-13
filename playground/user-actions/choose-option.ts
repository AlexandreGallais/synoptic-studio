import { selectInput } from "./select-input";

/**
 * Chooses an option of a select, as a user would: its `change` event.
 *
 * @kind procedure
 * @param id - id of the select, without `#`
 * @param value - the `value` attribute of the option to pick
 * @see docs/backlog/stories/E14-F11-EN-021-shared-test-helpers.md
 */
export function chooseOption(id: string, value: string): void {
  const select = selectInput(id);

  select.value = value;
  select.dispatchEvent(new Event("change", { bubbles: true }));
}
