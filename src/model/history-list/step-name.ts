import { drawingChangedFields } from "./drawing-changed-fields";
import { fieldsStepName } from "./fields-step-name";
import { isKeptInOrder } from "./is-kept-in-order";

import type { ShapeFields } from "./shape-fields";

/**
 * The name of a step by its action (US-077), read from the drawings before and after it: shapes
 * added, every other kept in order, `Add`; shapes removed, every other kept in order, `Delete`; as
 * many shapes, named by the fields changed (`drawingChangedFields`, `fieldsStepName`); any other
 * step, `Change`.
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each
 * @param before - the drawing before the step
 * @param after - the drawing after it
 * @param fields - reads the fields of a shape
 * @returns `Add`, `Delete`, `Move`, `Convert`, a field's words or `Change`
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function stepName<T>(
  before: readonly T[],
  after: readonly T[],
  fields: ShapeFields<T>,
): string {
  if (after.length === before.length) {
    return fieldsStepName(drawingChangedFields(before, after, fields));
  }

  const isAdded = after.length > before.length;
  const isKept = isAdded ? isKeptInOrder(before, after) : isKeptInOrder(after, before);

  if (!isKept) {
    return "Change";
  }

  return isAdded ? "Add" : "Delete";
}
