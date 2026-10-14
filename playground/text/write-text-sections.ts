import { selectElement } from "../page";

/** Sections of the shapes, hidden while the Text tool is chosen. */
const SHAPE_SECTIONS = ["size-section", "corners-section", "turn", "straight-box", "tools"];

/**
 * Shows the Text tool's fields and hides the shapes' size, corners, angle and modes while the
 * Text tool is chosen (a text turns with US-032), and the other way round; the position fields
 * stay, for the top-left of the text's block.
 *
 * @kind procedure
 * @param document - page document
 * @param isText - whether the Text tool is chosen
 * @see docs/backlog/stories/E15-F05-US-031-place-a-text.md
 */
export function writeTextSections(document: Document, isText: boolean): void {
  selectElement(document, "text-options").hidden = !isText;

  for (const id of SHAPE_SECTIONS) {
    selectElement(document, id).hidden = isText;
  }
}
