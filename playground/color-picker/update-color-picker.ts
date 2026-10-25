import { hexToPickedColor } from "../../src";
import { readColorField, readShownColor } from "../color-fields";

import { readColorModel } from "./read-color-model";
import { readOpenSwatch } from "./read-open-swatch";
import { readPickedColor } from "./read-picked-color";
import { showColorPicker } from "./show-color-picker";
import { writePickedColor } from "./write-picked-color";

/**
 * Reads the open picker's values again from its color, after a hexadecimal typed, kept exactly as typed: whole numbers in the model chosen, a grey keeping the hue the picker holds
 * (`hexToPickedColor`), the color kept as the one read; nothing while it is closed.
 *
 * @kind procedure
 * @param document - page document
 * @see US-035
 */
export function updateColorPicker(document: Document): void {
  const swatch = readOpenSwatch(document);

  if (swatch === undefined) {
    return;
  }

  const color = readShownColor(document, readColorField(swatch).color);
  const { hue } = readPickedColor(document);

  writePickedColor(document, hexToPickedColor(color, readColorModel(document), hue), color);
  showColorPicker(document);
}
