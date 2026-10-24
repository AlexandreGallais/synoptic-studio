import { parsePanelValue } from "../../src";
import { selectElement } from "../page";
import { readFieldValue } from "../text";

import { readArrowStep } from "./read-arrow-step";

import type { TypedPanelValue } from "../../src";

/**
 * Applies the step of an arrow pressed in a field acting as a number field (`readArrowStep`), its
 * default, the caret moved, prevented; another key is left to the field.
 *
 * @kind procedure
 * @param event - the key pressed in the field
 * @param apply - gives the step to what the field sets
 * @see docs/domain/interaction.md#_2-selector
 */
function applyArrowStep(event: KeyboardEvent, apply: (typed: TypedPanelValue) => void): void {
  const step = readArrowStep(event);

  if (step === undefined) {
    return;
  }

  event.preventDefault();
  apply(step);
}

/**
 * Makes a text field of the panel act as a number field (US-081, US-085), one rule for every such
 * field (`NUMBER_FIELDS`): a value typed is applied on Enter or when the field is left, never
 * while it is typed (`change`, `parsePanelValue`), an integer, relative with a sign; Up and Down
 * step it at their key by 1, or 10 with Shift, as `+1` or `-10` typed (`applyArrowStep`). What
 * the value sets, and how the field shows it again, is the caller's (`apply`).
 *
 * @kind procedure
 * @param document - page document
 * @param id - the field's id, without `#`
 * @param apply - gives the value, relative or fixed, `undefined` when the text typed is no
 * integer, to what the field sets, then shows it again
 * @see docs/domain/interaction.md#_2-selector
 */
export function listenToNumberField(
  document: Document,
  id: string,
  apply: (typed: TypedPanelValue | undefined) => void,
): void {
  const field = selectElement(document, id);

  field.addEventListener("change", () => {
    apply(parsePanelValue(readFieldValue(document, id)));
  });

  field.addEventListener("keydown", (event) => {
    applyArrowStep(event, apply);
  });
}
