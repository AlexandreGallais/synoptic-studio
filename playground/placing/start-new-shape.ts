import { NO_VERTEX_SELECTION, drawingWithShape } from "../../src";
import { DRAWING_KEYS, readStoredDrawing, writeStoredDrawing } from "../kept-shapes";
import { saveKindSettings, writeKindSettings } from "../kind-settings";
import { readShapeKind, selectElement, writeInputValue, writeSelection } from "../page";

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
 * Starts a new shape of a tool's kind, by a click (F35) or a drag (F36): the shape the panel
 * edited is kept below, unless it was refused; the panel then edits a new shape of the tool's
 * kind, straight, with the settings of the last shape of its kind (SP-026), its box written next.
 *
 * @kind procedure
 * @param document - page document
 * @param tool - the tool armed: `rectangle`, `polygon`, `round` or `text`
 * @see docs/domain/interaction.md#_1-tools
 */
export function startNewShape(document: Document, tool: string): void {
  addEditedShapeBelow(document);
  writeNewShapeKind(document, tool);
  writeKindSettings(document, tool);
  writeInputValue(document, "angle", 0);
}
