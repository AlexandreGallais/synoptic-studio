import { showHover } from "../hover";
import { readShapeKind } from "../page";
import { hidePointPanels, updateTextTool, writeTextSections } from "../text";
import { writeShapeKind } from "../toolbar";

import { updatePlayground } from "./update-playground";

/**
 * Redraws the page from its inputs: the Text tool, or the shapes; then the layer above the
 * drawing, so that the outline follows a shape changed under a pointer standing still (F36).
 *
 * @kind procedure
 * @param document - the playground's document, its fields read
 * @see docs/backlog/stories/E01-F01-US-002-sharp-rectangle-in-playground.md
 */
export function updatePage(document: Document): void {
  const isText = readShapeKind(document) === "text";

  writeTextSections(document, isText);

  if (isText) {
    writeShapeKind(document);
    hidePointPanels(document);
    updateTextTool(document);
  } else {
    updatePlayground(document);
  }

  showHover(document);
}
