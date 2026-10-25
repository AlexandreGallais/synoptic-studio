import { readSelectedShapes } from "../kept-shapes";
import { writeText } from "../page";

import { showNoSelection } from "./show-no-selection";
import { writeSeveralValues } from "./write-several-values";

/** What the panel says while several shapes are selected. */
const SEVERAL_SELECTED = "Shift + click a shape to add it or take it out, or click one to edit it.";

/**
 * Shows the drawing with several shapes selected: every shape drawn, in the order drawn,
 * none edited, as with none selected (`showNoSelection`); the panel's title saying how many are
 * selected, its status how to add or take one out, and their position and size in the fields of
 * several shapes (`writeSeveralValues`).
 *
 * @kind procedure
 * @param document - page document
 * @see RULE-interaction-2-selector
 */
export function showSeveralSelection(document: Document): void {
  showNoSelection(document);
  writeText(document, "shape-title", `${readSelectedShapes(document).length} shapes selected`);
  writeText(document, "status", SEVERAL_SELECTED);
  writeSeveralValues(document);
}
