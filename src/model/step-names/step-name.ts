import { drawingChangedFields } from "./drawing-changed-fields";
import { fieldsStepName } from "./fields-step-name";
import { isKeptInOrder } from "./is-kept-in-order";
import { isReordered } from "./is-reordered";

import type { ShapeFields } from "./shape-fields";

/**
 * The name of a step that kept as many shapes: `Change` when it moved shapes in the order
 * (`isReordered`), otherwise named by the fields it changed (`drawingChangedFields`,
 * `fieldsStepName`).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each
 * @param before - the drawing before the step
 * @param after - the drawing after it, as many shapes
 * @param fields - reads the fields of a shape
 * @returns `Move`, `Convert`, a field's label or `Change`
 * @see RULE-interaction-7-navigation-and-history
 */
function inPlaceStepName<T>(
  before: readonly T[],
  after: readonly T[],
  fields: ShapeFields<T>,
): string {
  return isReordered(before, after)
    ? "Change"
    : fieldsStepName(drawingChangedFields(before, after, fields));
}

/**
 * The name of a step by its action (US-077), read from the drawings before and after it: shapes
 * added, every other kept in order, `Add`; shapes removed, every other kept in order, `Delete`; as
 * many shapes, none moved in the order, named by the fields changed (`inPlaceStepName`); any
 * other step, shapes moved in the order among them, `Change`.
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each
 * @param before - the drawing before the step
 * @param after - the drawing after it
 * @param fields - reads the fields of a shape
 * @returns `Add`, `Delete`, `Move`, `Convert`, a field's label or `Change`
 * @see RULE-interaction-7-navigation-and-history
 */
export function stepName<T>(
  before: readonly T[],
  after: readonly T[],
  fields: ShapeFields<T>,
): string {
  if (after.length === before.length) {
    return inPlaceStepName(before, after, fields);
  }

  const isAdded = after.length > before.length;
  const isKept = isAdded ? isKeptInOrder(before, after) : isKeptInOrder(after, before);

  if (!isKept) {
    return "Change";
  }

  return isAdded ? "Add" : "Delete";
}
