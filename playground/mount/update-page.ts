import { showHover } from "../hover";
import { readShapeKind, selectElement } from "../page";
import { showNoSelection } from "../selection";
import { hidePointPanels, updateTextTool, writeTextSections } from "../text";
import { writeShapeKind } from "../toolbar";

import { updatePlayground } from "./update-playground";

/**
 * Redraws the shape the panel edits from its inputs: the Text tool, or the shapes.
 *
 * @kind procedure
 * @param document - the playground's document, its fields read
 * @param isText - whether the shape edited is a text
 * @see docs/backlog/stories/E01-F01-US-002-sharp-rectangle-in-playground.md
 */
function updateEditedShape(document: Document, isText: boolean): void {
  selectElement(document, "shape-panel").hidden = false;
  writeTextSections(document, isText);

  if (isText) {
    writeShapeKind(document);
    hidePointPanels(document);
    updateTextTool(document);
  } else {
    updatePlayground(document);
  }
}

/**
 * Redraws the page from its inputs: the Text tool, the shapes, or the drawing alone while no shape
 * is selected (`showNoSelection`, F36); then the layer above the drawing, so that the outlines
 * follow a shape changed under a pointer standing still (F36).
 *
 * @kind procedure
 * @param document - the playground's document, its fields read
 * @see docs/backlog/stories/E01-F01-US-002-sharp-rectangle-in-playground.md
 */
export function updatePage(document: Document): void {
  const kind = readShapeKind(document);

  if (kind === "none") {
    showNoSelection(document);
  } else {
    updateEditedShape(document, kind === "text");
  }

  showHover(document);
}
