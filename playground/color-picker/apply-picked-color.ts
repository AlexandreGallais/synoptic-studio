import { hexToExactColor, wholeValuesToHex } from "../../src";
import { readColorField, readColorInput, readShownColor, writeKeptColor } from "../color-fields";
import { selectElement, writeText } from "../page";

import { readColorModel } from "./read-color-model";
import { readOpenSwatch } from "./read-open-swatch";
import { readPickedColor } from "./read-picked-color";
import { showColorPicker } from "./show-color-picker";
import { writePickedColor } from "./write-picked-color";

import type { PickedColor } from "../../src";

/**
 * Applies whole values picked or typed in the open picker, kept on it: the color rebuilt from them
 * in the model chosen against the color its values were read from (`writePickedColor`), each value
 * equal to the one read standing for its exact value, each other kept as given
 * (`hexToExactColor`, `wholeValuesToHex`; ADR-0058, decisions 2 and 3), so that a value edited
 * never moves the others; the color written in the hexadecimal field, the page redrawn.
 *
 * @kind procedure
 * @param document - page document
 * @param picked - the hue, the saturation and the lightness or brightness, integers, one or two
 * of them changed
 * @param update - redraws the playground
 * @see docs/backlog/stories/E15-F16-US-040-whole-numbers-in-the-picker.md
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
  const model = readColorModel(document);
  const read =
    selectElement(document, "color-picker").dataset["readColor"] ??
    readShownColor(document, ids.color);
  const exact = hexToExactColor(read, model, readPickedColor(document).hue);
  const color = wholeValuesToHex(exact, model, picked);

  writePickedColor(document, picked, read);
  readColorInput(document, ids.color).value = color;
  writeKeptColor(document, ids.color);
  writeText(document, ids.message, "");
  update();
  showColorPicker(document);
}
