/**
 * A drawing sharing the shapes it did not change: each shape whose JSON text is the
 * present's shape's at the same index is replaced by that object, any other whose text is a shape's
 * of the present by the present's object, so that a state holds new objects only for the shapes
 * its action changed, and a drawing read again unchanged is the present's, shapes of equal texts
 * included (ADR-0095, decision 2; `REF-IMMER`).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each
 * @param drawing - the shapes recorded, in the order drawn
 * @param present - the shapes of the history's present
 * @returns a new drawing, in the same order, each shape the present's when their texts are equal
 * @see RULE-interaction-7-navigation-and-history
 */
export function sharedDrawing<T>(drawing: readonly T[], present: readonly T[]): readonly T[] {
  const texts = present.map((shape): readonly [string, T] => [JSON.stringify(shape), shape]);
  const byText = new Map(texts);

  return drawing.map((shape, index) => {
    const text = JSON.stringify(shape);
    const atIndex = texts[index];

    return atIndex?.[0] === text ? atIndex[1] : (byText.get(text) ?? shape);
  });
}
