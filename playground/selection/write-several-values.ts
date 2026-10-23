import { selectElement } from "../page";

/**
 * Shows the position and size of several shapes selected in the panel's four fields of several
 * shapes (F38): each shape's values (`shapePanelValues`, `textPanelValues`), a text never
 * measured, which is not drawn, left out; a value common to them as one shape's shows it, an
 * empty field reading Mixed where they differ (`commonPanelValues`), nor any placeholder with no
 * value left; read-only until US-071 brings the typing.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_2-selector
 */
export function writeSeveralValues(document: Document): void {
  selectElement(document, "shape-panel");
}
