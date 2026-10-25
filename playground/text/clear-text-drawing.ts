import { writeLeftOutFaces } from "../document";
import { DRAWING_KEYS, createDrawingSvg, writeStoredDrawing } from "../kept-shapes";
import { selectElement } from "../page";

/**
 * Clears what a refused text drew, the shapes below and above it kept: the text, its straight
 * box's line and the faces left out of its document; a refused text is not kept when
 * another shape is placed.
 *
 * @kind procedure
 * @param document - page document
 */
export function clearTextDrawing(document: Document): void {
  selectElement(document, "canvas").replaceChildren(createDrawingSvg(document, []));
  writeStoredDrawing(document, DRAWING_KEYS.edited, []);
  selectElement(document, "straight-box").hidden = true;
  writeLeftOutFaces(document, []);
}
