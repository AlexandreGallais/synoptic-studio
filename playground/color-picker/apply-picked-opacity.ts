import { readColorField } from "../color-fields";
import { writeInputValue } from "../page";

import { readOpenSwatch } from "./read-open-swatch";
import { showColorPicker } from "./show-color-picker";

/**
 * Applies an opacity picked on the open picker's slider: written in its color's opacity field,
 * the page redrawn, the picker shown again.
 *
 * @kind procedure
 * @param document - page document
 * @param opacity - the opacity, an integer from 0 to 100
 * @param update - redraws the playground
 * @see docs/backlog/stories/E15-F16-US-035-pick-a-color.md
 */
export function applyPickedOpacity(document: Document, opacity: number, update: () => void): void {
  const swatch = readOpenSwatch(document);

  if (swatch === undefined) {
    return;
  }

  writeInputValue(document, readColorField(swatch).opacity, opacity);
  update();
  showColorPicker(document);
}
