import { NO_VERTEX_SELECTION, drawingWithShape } from "../../src";
import { DRAWING_KEYS, readStoredDrawing, writeStoredDrawing } from "../kept-shapes";
import { saveKindSettings, writeKindSettings } from "../kind-settings";
import { readShapeKind, selectElement, writeInputValue, writeSelection } from "../page";
import { writeArmedTool } from "../toolbar";

import { writePlacedBox } from "./write-placed-box";
import { writePlacedText } from "./write-placed-text";

import type { ShapePlacement } from "./shape-placement";

/**
 * Keeps the shape the panel edits below the new one, unless it was refused, and its settings as
 * the last of its kind (`saveKindSettings`).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_1-tools
 */
function addEditedShapeBelow(document: Document): void {
  const [edited] = readStoredDrawing(document, DRAWING_KEYS.edited);

  if (edited === undefined) {
    return;
  }

  writeStoredDrawing(
    document,
    DRAWING_KEYS.below,
    drawingWithShape(readStoredDrawing(document, DRAWING_KEYS.below), edited),
  );

  writeStoredDrawing(document, DRAWING_KEYS.edited, []);
  saveKindSettings(document, readShapeKind(document));
}

/**
 * Makes the panel edit a new shape of a kind, without firing any event: no free shape kept, no
 * point selected.
 *
 * @kind procedure
 * @param document - page document
 * @param kind - `rectangle`, `polygon`, `round` or `text`
 * @see docs/domain/interaction.md#_1-tools
 */
function writeNewShapeKind(document: Document, kind: string): void {
  const select = selectElement(document, "shape");

  delete select.dataset["freeShape"];
  select.querySelector('option[value="free"]')?.setAttribute("hidden", "");
  Object.assign(select, { value: kind });
  writeSelection(document, NO_VERTEX_SELECTION);
}

/**
 * Places a shape by a click (F35): the shape the panel edited is kept below, unless it was
 * refused; the panel then edits a new shape of the tool's kind, straight, with the settings of
 * the last shape of its kind (SP-026), 100 × 100 or a text fitting its content, centred on the
 * point on integers; the selector is armed again, as in Figma.
 *
 * @kind procedure
 * @param document - page document
 * @param placement - the tool armed and the point clicked
 * @see docs/domain/interaction.md#_1-tools
 */
export function addPlacedShape(document: Document, placement: ShapePlacement): void {
  addEditedShapeBelow(document);
  writeNewShapeKind(document, placement.tool);
  writeKindSettings(document, placement.tool);
  writeInputValue(document, "angle", 0);

  if (placement.tool === "text") {
    writePlacedText(document, placement.point);
  } else {
    writePlacedBox(document, placement.point);
  }

  writeArmedTool(document, "selector");
}
