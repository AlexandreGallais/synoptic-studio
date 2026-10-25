import { readColorField, writeColorChosen } from "../color-fields";
import { writeInputValue } from "../page";

import { readOpenSwatch } from "./read-open-swatch";
import { showColorPicker } from "./show-color-picker";

/**
 * Applies an opacity picked on the open picker's slider: written in its color's opacity field,
 * the color marked chosen when it followed the text's (`writeColorChosen`), the page
 * redrawn, the picker shown again.
 *
 * @kind procedure
 * @param document - page document
 * @param opacity - the opacity, an integer from 0 to 100
 * @param update - redraws the playground
 */
export function applyPickedOpacity(document: Document, opacity: number, update: () => void): void {
  const swatch = readOpenSwatch(document);

  if (swatch === undefined) {
    return;
  }

  const ids = readColorField(swatch);

  writeInputValue(document, ids.opacity, opacity);
  writeColorChosen(document, ids);
  update();
  showColorPicker(document);
}
