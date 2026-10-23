import { showNoSelection } from "./show-no-selection";

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
}
