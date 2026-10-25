import { NUMBER_FIELD_APPLIES } from "./number-field-applies";

import type { TypedPanelValue } from "../../src";

/**
 * Makes a text field of the panel whose value sets the shapes act as a number field (US-081,
 * US-085), under the one rule of every such field (`NUMBER_FIELDS`, `listenToNumberFields`): a
 * value typed is applied on Enter or when the field is left, never while it is typed, a number,
 * relative with a sign; Up and Down step it at their key by 1, or 10 with Shift, as `+1` or `-10`
 * typed. What the value sets, and how the field shows it again, is the caller's (`apply`), told
 * whether it is an arrow's step, which one shape's X and Y land, kept for the field
 * (`NUMBER_FIELD_APPLIES`), a mount's replacing the one before.
 *
 * @kind procedure
 * @param document - page document
 * @param id - the field's id, without `#`
 * @param apply - gives the value, relative or fixed, `undefined` when the text typed is no
 * number, to what the field sets, then shows it again, `isStep` true for an arrow's step
 * @see RULE-interaction-2-selector
 */
export function listenToNumberField(
  document: Document,
  id: string,
  apply: (typed: TypedPanelValue | undefined, isStep: boolean) => void,
): void {
  const applies = NUMBER_FIELD_APPLIES.get(document) ?? new Map();

  NUMBER_FIELD_APPLIES.set(document, new Map([...applies, [id, apply]]));
}
