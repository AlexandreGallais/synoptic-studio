import { focusNewText } from "../placing";
import { writeArmedTool } from "../toolbar";

import { SHAPE_DRAWINGS } from "./shape-drawings";

/**
 * Ends the drag (F36), its last place already drawn: the shape drawn is kept, a text's content
 * selected so that typing replaces it (SP-026), and the selector armed again, as after a click; a
 * drag that never left the press's integer point leaves the release to the click (F35).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_1-tools
 */
export function stopShapeDrawing(document: Document): void {
  const drawing = SHAPE_DRAWINGS.get(document);

  SHAPE_DRAWINGS.delete(document);

  if (drawing?.isDrawn !== true) {
    return;
  }

  if (drawing.tool === "text") {
    focusNewText(document);
  }

  writeArmedTool(document, "selector");
}
