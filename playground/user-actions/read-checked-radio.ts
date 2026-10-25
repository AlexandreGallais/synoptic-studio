/**
 * Value of the checked radio button of a group, read by property: happy-dom's `:checked` follows
 * the `checked` attribute, not the state.
 *
 * @kind procedure
 * @param name - `name` of the group
 * @returns the value checked, empty when none is
 * @see EN-021
 */
export function readCheckedRadio(name: string): string {
  return (
    [...document.querySelectorAll<HTMLInputElement>(`input[name="${CSS.escape(name)}"]`)].find(
      (button) => button.checked,
    )?.value ?? ""
  );
}
