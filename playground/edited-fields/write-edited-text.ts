import { writeInputValue } from "../page";

import { writePickedTextPosition } from "./write-picked-text-position";

import type { Text } from "../../src";

/**
 * Writes a text the panel edits alone, set by its X, Y or its box's Width or Height, into
 * the page: its box's size in its fields, then its place, X and Y showing where its straight box
 * starts and the text kept as the one last drawn, so that the redraw finds it exact
 * (`writePickedTextPosition`).
 *
 * @kind procedure
 * @param document - page document
 * @param text - the text set, or as it was for a value refused
 */
export function writeEditedText(document: Document, text: Text): void {
  writeInputValue(document, "text-box-width", text.boxWidth);
  writeInputValue(document, "text-box-height", text.boxHeight);
  writePickedTextPosition(document, text);
}
