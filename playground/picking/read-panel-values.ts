import { isDrawnText, shapePanelValues, textPanelValues } from "../../src";

import { readTextBlock } from "./read-text-block";

import type { DrawingShape, PanelValues } from "../../src";

/**
 * Reads the values the panel shows for a shape of the drawing (`shapePanelValues`,
 * `textPanelValues`), a text by its block as it is drawn (`readTextBlock`).
 *
 * @kind procedure
 * @param document - page document
 * @param shape - a shape of the drawing
 * @returns its values; none for a text never measured, which is not drawn
 * @see RULE-interaction-2-selector
 */
export function readPanelValues(document: Document, shape: DrawingShape): readonly PanelValues[] {
  if (!isDrawnText(shape)) {
    return [shapePanelValues(shape)];
  }

  const block = readTextBlock(document, shape);

  return block === undefined ? [] : [textPanelValues(shape, block)];
}
