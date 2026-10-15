import { hexToPickedColor } from "../../src";
import { readColorField, readShownColor } from "../color-fields";
import { selectElement } from "../page";

import { closeColorPicker } from "./close-color-picker";
import { readColorModel } from "./read-color-model";
import { showColorPicker } from "./show-color-picker";
import { writePickedColor } from "./write-picked-color";

/** The hue a picker opens with for a grey, which has none of its own (ADR-0056). */
const GREY_HUE = 0;

/**
 * Opens the picker under a swatch's fields, closing it first where it was: its values read from
 * the color in the model chosen, to one decimal, a grey at the hue 0 (`hexToPickedColor`).
 *
 * @kind procedure
 * @param document - page document
 * @param swatch - the swatch clicked
 * @see docs/backlog/stories/E15-F16-US-035-pick-a-color.md
 */
export function openColorPicker(document: Document, swatch: HTMLElement): void {
  const picker = selectElement(document, "color-picker");
  const ids = readColorField(swatch);
  const color = readShownColor(document, ids.color);

  closeColorPicker(document);
  writePickedColor(document, hexToPickedColor(color, readColorModel(document), GREY_HUE));
  selectElement(document, ids.message).after(picker);
  picker.hidden = false;
  swatch.setAttribute("aria-expanded", "true");
  showColorPicker(document);
}
