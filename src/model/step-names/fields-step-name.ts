import { fieldLabel } from "./field-label";

/** The fields of a shape's position, that a move changes alone. */
const POSITION_FIELDS: ReadonlySet<string> = new Set(["x", "y"]);

/**
 * The name of a step that changed shapes in place, by the fields it changed: `kind`
 * among them, `Convert`; only `x` and `y`, `Move`; one other field beside them, its label
 * (`fieldLabel`), so that a width typed with the anchor moving the shape is `Width`; none, or
 * several others, `Change`.
 *
 * @kind domain
 * @param fields - the names of the fields the step changed (`drawingChangedFields`)
 * @returns `Convert`, `Move`, a field's label or `Change`
 * @see RULE-interaction-7-navigation-and-history
 */
export function fieldsStepName(fields: readonly string[]): string {
  const others = fields.filter((field) => !POSITION_FIELDS.has(field));

  if (fields.includes("kind")) {
    return "Convert";
  }

  if (fields.length > 0 && others.length === 0) {
    return "Move";
  }

  return others.length === 1 ? fieldLabel(others.join("")) : "Change";
}
