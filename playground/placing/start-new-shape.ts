import {
  DRAWING_KEYS,
  readKeptDrawing,
  readStoredDrawing,
  writeStoredDrawing,
} from "../kept-shapes";
import { saveKindSettings, writeKindSettings } from "../kind-settings";
import { readShapeKind, writeEditedKind, writeInputValue } from "../page";

/**
 * Keeps every shape of the drawing below the new one, in the order drawn (`readKeptDrawing`): the
 * shape the panel edits, unless it was refused, keeping its place among the others (F36), and its
 * settings as the last of its kind (`saveKindSettings`).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_1-tools
 */
function addDrawingBelow(document: Document): void {
  const isEdited = readStoredDrawing(document, DRAWING_KEYS.edited).length > 0;

  writeStoredDrawing(document, DRAWING_KEYS.below, readKeptDrawing(document));
  writeStoredDrawing(document, DRAWING_KEYS.edited, []);
  writeStoredDrawing(document, DRAWING_KEYS.above, []);

  if (isEdited) {
    saveKindSettings(document, readShapeKind(document));
  }
}

/**
 * Starts a new shape of a tool's kind, by a click (F35) or a drag (F36): every shape of the
 * drawing is kept below it, the one the panel edited in its place unless it was refused, so that
 * the new shape goes on top of all; the panel then edits a new shape of the tool's kind, straight,
 * with the settings of the last shape of its kind (SP-026), its box written next.
 *
 * @kind procedure
 * @param document - page document
 * @param tool - the tool armed: `rectangle`, `polygon`, `round` or `text`
 * @see docs/domain/interaction.md#_1-tools
 */
export function startNewShape(document: Document, tool: string): void {
  addDrawingBelow(document);
  writeEditedKind(document, tool);
  writeKindSettings(document, tool);
  writeInputValue(document, "angle", 0);
}
