import { writePickedTextPosition } from "../edited-fields";
import { writeEditedKind } from "../page";
import { writeTextFields } from "../shape-fields";

import type { Text } from "editor";

/**
 * Loads a text picked into the panel, without firing any event: the Text tool chosen, and
 * its whole style written into the Text tool's fields (`writeTextFields`) — its content, family
 * chosen in the font list, which the font session then measures with, weight, italic, strokes,
 * box, background and every other option, as typing next is likely meant in that style — and its
 * place (`writePickedTextPosition`).
 *
 * @kind procedure
 * @param document - page document
 * @param text - a text of the drawing, clicked
 */
export function loadPickedText(document: Document, text: Text): void {
  writeEditedKind(document, "text");
  writeTextFields(document, text);
  writePickedTextPosition(document, text);
}
