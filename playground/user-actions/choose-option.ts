import { selectChoice } from "./select-choice";

/**
 * Chooses an option of a select, as a user would: its `change` event.
 *
 * @kind procedure
 * @param id - id of the select, without `#`
 * @param value - the `value` attribute of the option to pick
 * @throws {Error} when the select has no such option: the test would go on with none chosen
 * @see docs/backlog/stories/E14-F11-EN-021-shared-test-helpers.md
 */
export function chooseOption(id: string, value: string): void {
  const select = selectChoice(id);

  select.value = value;

  if (select.value !== value) {
    throw new Error(`#${id} has no option ${value}.`);
  }

  select.dispatchEvent(new Event("change", { bubbles: true }));
}
