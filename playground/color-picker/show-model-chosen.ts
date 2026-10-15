import { hexToPickedColor, pickedColorToHex } from "../../src";
import { readColorField, readShownColor } from "../color-fields";

import { readColorModel } from "./read-color-model";
import { readOpenSwatch } from "./read-open-swatch";
import { readPickedColor } from "./read-picked-color";
import { showColorPicker } from "./show-color-picker";
import { writePickedColor } from "./write-picked-color";

/**
 * Shows the open picker in the model just chosen: the saturation and the lightness or brightness
 * read again from its color (`hexToPickedColor`); the hue it holds kept while the three values
 * still give the color back (`pickedColorToHex`), a grey always, as an open picker keeps its own
 * hue (ADR-0056, decision 3), else the color's own hue, so that typed back they give the color
 * again (decision 4; AUD-017, provisional); the color unchanged.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E15-F16-US-035-pick-a-color.md
 */
export function showModelChosen(document: Document): void {
  const swatch = readOpenSwatch(document);

  if (swatch === undefined) {
    return;
  }

  const color = readShownColor(document, readColorField(swatch).color);
  const model = readColorModel(document);
  const { hue } = readPickedColor(document);
  const shown = hexToPickedColor(color, model, hue);
  const kept = { ...shown, hue };

  writePickedColor(document, pickedColorToHex(kept, model) === color ? kept : shown);
  showColorPicker(document);
}
