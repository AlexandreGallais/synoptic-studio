import { selectInput } from "./select-input";
import { typeAndEnter } from "./type-and-enter";

/**
 * Types a value with Enter into a field as a designer does: the field given the focus, a key
 * pressed in it before the text changes, then the text typed and Enter, so that a field acting as
 * a number field reads the number it showed before, which a relative value starts from and a
 * value refused puts back (US-087).
 *
 * @kind procedure
 * @param id - input id
 * @param value - typed text
 * @see docs/domain/interaction.md#_2-selector
 */
export function typeInField(id: string, value: string): void {
  const input = selectInput(id);

  input.blur();
  input.focus();

  input.dispatchEvent(
    new KeyboardEvent("keydown", { bubbles: true, cancelable: true, key: value.at(0) ?? "Delete" }),
  );

  typeAndEnter(id, value);
}
