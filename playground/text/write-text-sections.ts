import { selectElement } from "../page";

/** Sections of the shapes hidden while the Text tool is chosen, shown again when it is left. */
const SHAPE_SECTIONS = ["size-section", "corners-section", "straight-box", "tools"];

/**
 * Shows the Text tool's fields and hides every control of the shapes while the Text tool is
 * chosen: size, corners, straight box and modes, and the other way round; the angle and position
 * fields stay, for the text's angle and the top-left of its block (US-032).
 *
 * @kind procedure
 * @param document - page document
 * @param isText - whether the Text tool is chosen
 * @see US-031
 */
export function writeTextSections(document: Document, isText: boolean): void {
  selectElement(document, "text-options").hidden = !isText;

  for (const id of SHAPE_SECTIONS) {
    selectElement(document, id).hidden = isText;
  }
}
