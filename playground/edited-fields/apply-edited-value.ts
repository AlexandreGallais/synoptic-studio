import type { PanelValues, TypedPanelValue } from "../../src";

/**
 * Sets the shape the panel edits, alone, by a value of one of its fields (US-085), as each of
 * several shapes is set (`readEditedKept`, `shapeAfterPanelEdit`, `textAfterPanelEdit`): typed, an
 * integer relative with a sign (`parsePanelValue`), or an arrow's step, relative, from the shape
 * as last drawn, a size capped at 100 000 (Q20); the shape then written into the panel and its
 * fields showing its values again (`writeEditedGeometry`, `writeEditedText`), a value refused
 * putting them back. A shape refused has no value to start from: its field keeps the text typed,
 * read as before.
 *
 * @kind procedure
 * @param document - page document
 * @param key - the field: `x`, `y`, `width` or `height`
 * @param typed - the value, relative or fixed; `undefined` when the text typed is no integer
 * @see docs/domain/interaction.md#_2-selector
 */
export function applyEditedValue(
  document: Document,
  key: keyof PanelValues,
  typed: TypedPanelValue | undefined,
): void {
  // Placeholder of US-085.T1, setting nothing: the code comes with T2.
  if (typed === undefined) {
    document.dispatchEvent(new CustomEvent(key));
  }
}
