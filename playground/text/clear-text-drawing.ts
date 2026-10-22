import { writeLeftOutFaces } from "../document";
import { DRAWING_KEYS, createDrawingSvg, writeStoredDrawing } from "../kept-shapes";
import { selectElement } from "../page";

/**
 * Clears what a refused text drew, the shapes placed before it kept (F35): the text, its straight
 * box's line (US-039) and the faces left out of its document; a refused text is not kept when
 * another shape is placed.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E15-F14-US-039-straight-box-of-a-turned-text.md
 */
export function clearTextDrawing(document: Document): void {
  selectElement(document, "canvas").replaceChildren(createDrawingSvg(document));
  writeStoredDrawing(document, DRAWING_KEYS.edited, []);
  selectElement(document, "straight-box").hidden = true;
  writeLeftOutFaces(document, []);
}
