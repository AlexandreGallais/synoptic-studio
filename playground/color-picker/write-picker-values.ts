import { writeInputValue, writeText } from "../page";

import type { ColorModel, PickedColor } from "../../src";

/**
 * Shows the picker's three values, whole numbers as it holds them: the hue, the saturation and
 * the lightness (L) or brightness (B) of the model chosen.
 *
 * @kind procedure
 * @param document - page document
 * @param picked - the values the picker holds
 * @param model - the model chosen
 * @see US-035
 */
export function writePickerValues(
  document: Document,
  picked: PickedColor,
  model: ColorModel,
): void {
  writeText(document, "color-level-name", model === "hsb" ? "B" : "L");
  writeInputValue(document, "color-hue-value", picked.hue);
  writeInputValue(document, "color-saturation-value", picked.saturation);
  writeInputValue(document, "color-level-value", picked.level);
}
