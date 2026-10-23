/**
 * A drawing sharing the shapes it did not change (F40): each shape whose JSON text is a shape's
 * of the present is replaced by the present's object, so that a state holds new objects only for
 * the shapes its action changed (ADR-0095, decision 2; `REF-IMMER`).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each
 * @param drawing - the shapes recorded, in the order drawn
 * @param present - the shapes of the history's present
 * @returns a new drawing, in the same order, each shape the present's when their texts are equal
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function sharedDrawing<T>(drawing: readonly T[], present: readonly T[]): readonly T[] {
  const byText = new Map(present.map((shape) => [JSON.stringify(shape), shape]));

  return drawing.map((shape) => byText.get(JSON.stringify(shape)) ?? shape);
}
