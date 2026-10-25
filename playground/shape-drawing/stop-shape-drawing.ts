import { CLICKED_PRESSES, focusNewText } from "../placing";
import { writeArmedTool } from "../toolbar";

import { SHAPE_DRAWINGS } from "./shape-drawings";

/**
 * Leaves the shape a drag drew to the designer: a text's content selected so that typing replaces
 * it, and the selector armed again, as after a click.
 *
 * @kind procedure
 * @param document - page document
 * @param tool - the tool that drew it
 * @see RULE-interaction-1-tools
 */
function focusDrawnShape(document: Document, tool: string): void {
  if (tool === "text") {
    focusNewText(document);
  }

  writeArmedTool(document, "selector");
}

/**
 * Ends the drag, its last place already drawn: the shape drawn is kept, a text's content
 * selected so that typing replaces it, and the selector armed again, as after a click; a
 * press whose pointer stayed within 3 pixels of it leaves the release to the click, which places
 * the shape centred on the point pressed (`CLICKED_PRESSES`, SP-030's answer 8).
 *
 * @kind procedure
 * @param document - page document
 * @see RULE-interaction-1-tools
 */
export function stopShapeDrawing(document: Document): void {
  const drawing = SHAPE_DRAWINGS.get(document);

  SHAPE_DRAWINGS.delete(document);

  if (drawing === undefined) {
    return;
  }

  if (drawing.isDrawn) {
    focusDrawnShape(document, drawing.tool);
  } else {
    CLICKED_PRESSES.set(document, drawing.press);
  }
}
