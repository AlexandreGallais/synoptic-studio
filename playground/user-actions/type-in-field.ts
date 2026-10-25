import { commitField } from "./commit-field";
import { selectInput } from "./select-input";
import { typeValue } from "./type-value";

/**
 * Presses a key in a field as the browser sends it: bubbling, cancelable.
 *
 * @kind procedure
 * @param input - the field
 * @param key - the key's name
 * @see RULE-interaction-2-selector
 */
function pressKeyIn(input: HTMLInputElement, key: string): void {
  input.dispatchEvent(new KeyboardEvent("keydown", { bubbles: true, cancelable: true, key }));
}

/**
 * Types a value with Enter into a field as a designer does and a browser sends it: the
 * field given the focus, its text emptied, then for each character a key pressed before the text
 * changes and its `input`, then Enter and the `change` it applies, so that a field acting as a
 * number field reads the number it showed before at each key, which a relative value starts from
 * and a value refused puts back.
 *
 * @kind procedure
 * @param id - input id
 * @param value - typed text
 * @see RULE-interaction-2-selector
 */
export function typeInField(id: string, value: string): void {
  const input = selectInput(id);

  input.blur();
  input.focus();
  pressKeyIn(input, "Delete");
  typeValue(id, "");

  const indexes = Array.from({ length: value.length }).keys();

  for (const index of indexes) {
    pressKeyIn(input, value.charAt(index));
    typeValue(id, value.slice(0, index + 1));
  }

  pressKeyIn(input, "Enter");
  commitField(id);
}
