import { DRAWING_KEYS, writeStoredDrawing } from "../kept-shapes";

import { NEW_TEXTS } from "./new-texts";
import { readNewText } from "./read-new-text";

/**
 * Ends the typing of the page's new text (`NEW_TEXTS`, F40): a text still empty is removed
 * (`readNewText`), as a refused shape is not kept, so that the drawing is back as it was before
 * it was placed; a text typed stays, its step taken at the next one the page records. Called
 * when the panel leaves the text, before it reads the drawing, and before an undo or a redo.
 *
 * @kind procedure
 * @param document - page document
 * @see RULE-interaction-1-tools
 */
export function stopNewText(document: Document): void {
  const text = readNewText(document);

  NEW_TEXTS.delete(document);

  if (text?.content === "") {
    writeStoredDrawing(document, DRAWING_KEYS.edited, []);
  }
}
