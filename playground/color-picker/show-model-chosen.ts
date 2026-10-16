import { hexToPickedColor } from "../../src";
import { readColorField, readShownColor } from "../color-fields";

import { readColorModel } from "./read-color-model";
import { readOpenSwatch } from "./read-open-swatch";
import { readPickedColor } from "./read-picked-color";
import { showColorPicker } from "./show-color-picker";
import { writePickedColor } from "./write-picked-color";

/**
 * Shows the open picker in the model just chosen: its whole values read again from its color
 * (`hexToPickedColor`), kept as the one read, the color's own hue, the same in both models, or for a grey the hue the
 * picker holds (ADR-0056, decision 3; ADR-0058); the color unchanged.
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

  writePickedColor(document, hexToPickedColor(color, model, hue), color);
  showColorPicker(document);
}
