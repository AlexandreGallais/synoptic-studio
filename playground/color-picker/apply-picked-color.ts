import { pickedColorToHex } from "../../src";
import { readColorField, readColorInput, writeKeptColor } from "../color-fields";
import { writeText } from "../page";

import { readColorModel } from "./read-color-model";
import { readOpenSwatch } from "./read-open-swatch";
import { showColorPicker } from "./show-color-picker";
import { writePickedColor } from "./write-picked-color";

import type { PickedColor } from "../../src";

/**
 * Applies values picked or typed in the open picker: kept on it, its hue among them, and the
 * color they give in the model chosen written in the hexadecimal field (`pickedColorToHex`); the
 * page redrawn, the picker shown again.
 *
 * @kind procedure
 * @param document - page document
 * @param picked - the hue, the saturation and the lightness or brightness, to one decimal
 * @param update - redraws the playground
 * @see docs/backlog/stories/E15-F16-US-035-pick-a-color.md
 */
export function applyPickedColor(
  document: Document,
  picked: PickedColor,
  update: () => void,
): void {
  const swatch = readOpenSwatch(document);

  if (swatch === undefined) {
    return;
  }

  const ids = readColorField(swatch);

  writePickedColor(document, picked);
  readColorInput(document, ids.color).value = pickedColorToHex(picked, readColorModel(document));
  writeKeptColor(document, ids.color);
  writeText(document, ids.message, "");
  update();
  showColorPicker(document);
}
