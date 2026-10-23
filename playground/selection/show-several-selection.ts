import { readSelectedShapes } from "../kept-shapes";
import { writeText } from "../page";

import { showNoSelection } from "./show-no-selection";

/** What the panel says while several shapes are selected, their values coming with F38. */
const SEVERAL_SELECTED = "Shift + click a shape to add it or take it out, or click one to edit it.";

/**
 * Shows the drawing with several shapes selected (F37): every shape drawn, in the order drawn,
 * none edited, as with none selected (`showNoSelection`); the panel's title saying how many are
 * selected, its status how to add or take one out; their values come with F38.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_2-selector
 */
export function showSeveralSelection(document: Document): void {
  showNoSelection(document);
  writeText(document, "shape-title", `${readSelectedShapes(document).length} shapes selected`);
  writeText(document, "status", SEVERAL_SELECTED);
}
