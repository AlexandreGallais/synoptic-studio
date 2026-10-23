import { writeText } from "../page";

/**
 * Selects shapes by their indices in the drawing (F37): the panel leaves the shape it edited,
 * keeping its settings as the last of its kind (`saveKindSettings`); one shape is split around and
 * loaded into the panel, as a click loads it (`splitDrawing`, `loadPickedShape`); several are kept
 * with every shape below, none edited, their indices beside them (`DRAWING_KEYS.selected`), the
 * kind `several`; none leaves every shape below, the kind `none`.
 *
 * @kind procedure
 * @param document - page document
 * @param selected - the indices of the shapes to select, ascending, in the order drawn
 * @see docs/domain/interaction.md#_2-selector
 */
export function writeShapeSelection(document: Document, selected: readonly number[]): void {
  // Placeholder until US-068.T2.
  writeText(document, "status", selected.join(" "));
}
