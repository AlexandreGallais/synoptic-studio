import { listenToNumberField } from "../number-fields";

import { applyEditedValue } from "./apply-edited-value";
import { applySteppedPosition } from "./apply-stepped-position";
import { EDITED_FIELDS } from "./edited-fields";

import type { EditedField } from "./edited-field";
import type { TypedPanelValue } from "../../src";

/** A value applied in a field of the shape the panel edits alone. */
type AppliedValue = {
  /** Whether it is an arrow's step rather than a value typed. */
  readonly isStep: boolean;
  /** The value, relative or fixed; `undefined` when the text typed is no number. */
  readonly typed: TypedPanelValue | undefined;
};

/** The fields an arrow's step moves the shape in as the arrow on the drawing, by id. */
const STEPPED_AXES: ReadonlyMap<string, "x" | "y"> = new Map([
  ["shape-x", "x"],
  ["shape-y", "y"],
]);

/**
 * Reads the axis an arrow's step in a field moves the shape along as the arrow on the drawing:
 * X's or Y's (`STEPPED_AXES`).
 *
 * @kind procedure
 * @param field - the value it sets and the field's id
 * @param isStep - whether the value is an arrow's step
 * @returns the axis; undefined for a value typed, or a step in Width or Height
 * @see RULE-interaction-2-selector
 */
function readSteppedAxis(field: EditedField, isStep: boolean): "x" | "y" | undefined {
  return isStep ? STEPPED_AXES.get(field[1]) : undefined;
}

/**
 * Applies a value to a field of the shape the panel edits alone: an arrow's step in X or Y moves
 * it as the arrow on the drawing, landed on integers (`applySteppedPosition`); any other
 * value, and a step in its Width or Height, is set by the computation of several shapes'
 * (`applyEditedValue`).
 *
 * @kind procedure
 * @param document - page document
 * @param field - the value it sets and the field's id
 * @param applied - the value and whether it is an arrow's step
 * @see RULE-interaction-2-selector
 */
function applyFieldValue(document: Document, field: EditedField, applied: AppliedValue): void {
  const axis = readSteppedAxis(field, applied.isStep);
  const { typed } = applied;

  if (axis !== undefined && typed !== undefined) {
    applySteppedPosition(document, axis, typed.value);

    return;
  }

  applyEditedValue(document, field, typed);
}

/**
 * Makes the fields of the shape the panel edits, alone, act as number fields, as several
 * shapes' (`listenToNumberField`, `EDITED_FIELDS`): a value typed applied on Enter or when the
 * field is left, Up and Down stepping it by 1, or 10 with Shift, each set by the computation of
 * several shapes', but for an arrow in X or Y, which moves it as the arrow on the drawing, landed
 * on integers (`applyFieldValue`); the page is then redrawn.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page
 * @see RULE-interaction-2-selector
 */
export function listenToEditedFields(document: Document, update: () => void): void {
  for (const field of EDITED_FIELDS) {
    listenToNumberField(document, field[1], (typed, isStep) => {
      applyFieldValue(document, field, { isStep, typed });
      update();
    });
  }
}
