/**
 * Clicks the radio button of a group that carries a value, as a user would: checked, then its
 * `change` event.
 *
 * @kind procedure
 * @param name - `name` of the group
 * @param value - `value` of the button to check
 * @throws {Error} when the group has no button of that value
 * @see docs/backlog/stories/E14-F11-EN-021-shared-test-helpers.md
 */
export function chooseRadio(name: string, value: string): void {
  const button = [
    ...document.querySelectorAll<HTMLInputElement>(`input[name="${CSS.escape(name)}"]`),
  ].find((cell) => cell.value === value);

  if (button === undefined) {
    throw new Error(`The group ${name} has no button ${value}.`);
  }

  button.click();
}
