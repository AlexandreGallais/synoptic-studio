import { splitDrawing } from "../../src";
import {
  DRAWING_KEYS,
  readKeptDrawing,
  readStoredDrawing,
  writeStoredDrawing,
} from "../kept-shapes";
import { saveKindSettings } from "../kind-settings";
import { readShapeKind, selectElement, writeEditedKind } from "../page";

import { loadPickedShape } from "./load-picked-shape";

import type { KeptShape } from "../kept-shapes";

/**
 * Keeps the settings of the shape the panel leaves as the last of its kind (`saveKindSettings`),
 * unless none is edited or it was refused.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_1-tools
 */
function saveEditedSettings(document: Document): void {
  if (readStoredDrawing(document, DRAWING_KEYS.edited).length > 0) {
    saveKindSettings(document, readShapeKind(document));
  }
}

/**
 * Makes the panel show the shapes selected: the one split around loaded into it, as a click loads
 * it (`loadPickedShape`); otherwise the kind `none`, or `several` while two or more are selected.
 *
 * @kind procedure
 * @param document - page document
 * @param edited - the shape split around, none for several or none selected
 * @param count - how many shapes are selected
 * @see docs/domain/interaction.md#_2-selector
 */
function writeSelectedKind(document: Document, edited: readonly KeptShape[], count: number): void {
  const [picked] = edited;

  if (picked === undefined) {
    writeEditedKind(document, count === 0 ? "none" : "several");
  } else {
    loadPickedShape(document, picked);
  }
}

/**
 * Selects shapes by their indices in the drawing (F37): the panel leaves the shape it edited,
 * keeping its settings as the last of its kind (`saveEditedSettings`); one shape is split around
 * and loaded into the panel, as a click loads it (`splitDrawing`, `writeSelectedKind`); several
 * are kept with every shape below, none edited, their indices beside them
 * (`DRAWING_KEYS.selected`), the kind `several`; none leaves every shape below, the kind `none`.
 *
 * @kind procedure
 * @param document - page document
 * @param selected - the indices of the shapes to select, ascending, in the order drawn
 * @see docs/domain/interaction.md#_2-selector
 */
export function writeShapeSelection(document: Document, selected: readonly number[]): void {
  saveEditedSettings(document);

  // Only one shape is split around; several, or none, leave every shape below.
  const [only] = selected.length === 1 ? selected : [];
  const { above, below, edited } = splitDrawing(readKeptDrawing(document), only ?? -1);

  writeStoredDrawing(document, DRAWING_KEYS.below, below);
  writeStoredDrawing(document, DRAWING_KEYS.edited, edited);
  writeStoredDrawing(document, DRAWING_KEYS.above, above);
  selectElement(document, "canvas").dataset[DRAWING_KEYS.selected] = selected.join(" ");
  writeSelectedKind(document, edited, selected.length);
}
