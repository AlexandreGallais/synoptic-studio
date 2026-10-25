/**
 * The fields two versions of a shape differ by: each field either holds whose JSON texts
 * differ, a field one holds alone included, in the order of their names.
 *
 * @kind domain
 * @param before - the shape's fields before a step
 * @param after - its fields after it
 * @returns the names of the fields changed, sorted; none when the two are equal
 * @see RULE-interaction-7-navigation-and-history
 */
export function changedFields(
  before: Readonly<Record<string, unknown>>,
  after: Readonly<Record<string, unknown>>,
): readonly string[] {
  return Object.keys({ ...before, ...after })
    .filter((field) => JSON.stringify(before[field]) !== JSON.stringify(after[field]))
    .toSorted((first, second) => first.localeCompare(second));
}
