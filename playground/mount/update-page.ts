import { showHover } from "../hover";
import { readShapeKind, selectElement } from "../page";
import { showNoSelection, showSeveralSelection } from "../selection";
import { hidePointPanels, updateTextTool, writeTextSections } from "../text";
import { writeShapeKind } from "../toolbar";
import { applyWheelZoom } from "../view";
import { showZoom } from "../zoom";

import { updatePlayground } from "./update-playground";

/**
 * Redraws the shape the panel edits from its inputs: the Text tool, or the shapes.
 *
 * @kind procedure
 * @param document - the playground's document, its fields read
 * @param isText - whether the shape edited is a text
 * @see US-002
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
 * Redraws the drawing alone, no shape edited: none selected (`showNoSelection`, F36), or several
 * (`showSeveralSelection`, F37).
 *
 * @kind procedure
 * @param document - the playground's document
 * @param kind - `none` or `several`
 * @see RULE-interaction-2-selector
 */
function showNoShapeEdited(document: Document, kind: string): void {
  if (kind === "none") {
    showNoSelection(document);
  } else {
    showSeveralSelection(document);
  }
}

/**
 * Redraws the page from its inputs, the view first zoomed by the wheel's deltas gathered since the
 * last frame (`applyWheelZoom`, F42), its percentage shown in the toolbar (`showZoom`): the Text tool, the shapes, or the drawing alone while none or
 * several shapes are selected (`showNoShapeEdited`, F36, F37), the Points button disabled and the
 * fields of several shapes shown while several are (F38), Points mode editing one shape; then the
 * layer above the drawing, so that the
 * outlines follow a shape changed under a pointer standing still (F36).
 *
 * @kind procedure
 * @param document - the playground's document, its fields read
 * @see US-002
 */
export function updatePage(document: Document): void {
  applyWheelZoom(document);
  showZoom(document);

  const kind = readShapeKind(document);

  selectElement(document, "mode-points").toggleAttribute("disabled", kind === "several");
  selectElement(document, "several-fields").hidden = kind !== "several";

  if (kind === "none" || kind === "several") {
    showNoShapeEdited(document, kind);
  } else {
    updateEditedShape(document, kind === "text");
  }

  showHover(document);
}
