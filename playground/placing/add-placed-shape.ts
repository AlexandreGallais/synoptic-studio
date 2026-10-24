import { writeArmedTool } from "../toolbar";

import { focusNewText } from "./focus-new-text";
import { startNewShape } from "./start-new-shape";
import { writeClickedShape } from "./write-clicked-shape";

import type { ShapePlacement } from "./shape-placement";

/**
 * Places a shape by a click (F35): a new shape of the tool's kind (`startNewShape`), 100 × 100 or
 * a text fitting its content, centred on the point on integers (`writeClickedShape`), a text's
 * content selected; the selector is armed again, as in Figma.
 *
 * @kind procedure
 * @param document - page document
 * @param placement - the tool armed and the point clicked
 * @see docs/domain/interaction.md#_1-tools
 */
export function addPlacedShape(document: Document, placement: ShapePlacement): void {
  startNewShape(document, placement.tool);
  writeClickedShape(document, placement);

  if (placement.tool === "text") {
    focusNewText(document);
  }

  writeArmedTool(document, "selector");
}
