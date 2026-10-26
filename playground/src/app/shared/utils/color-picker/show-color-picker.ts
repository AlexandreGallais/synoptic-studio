import { DEGREES_PER_TURN, PERCENT_SLIDER_RANGE } from "editor";

import { readColorField, readShownColor } from "../color-fields";
import { readNumber, selectElement } from "../page";

import { readColorModel } from "./read-color-model";
import { readOpenSwatch } from "./read-open-swatch";
import { readPickedColor } from "./read-picked-color";
import { writeColorPlane } from "./write-color-plane";
import { writePickerValues } from "./write-picker-values";
import { writeSliderThumb } from "./write-slider-thumb";

/**
 * Shows the open picker from the values it holds and its color's fields: the hue slider, the
 * opacity slider painted with the color, the plane and the three values; nothing while it is
 * closed.
 *
 * @kind procedure
 * @param document - page document
 */
export function showColorPicker(document: Document): void {
  const swatch = readOpenSwatch(document);

  if (swatch === undefined) {
    return;
  }

  const ids = readColorField(swatch);
  const picked = readPickedColor(document);
  const model = readColorModel(document);
  const opacity = readNumber(document, ids.opacity);
  const hue = String(picked.hue);

  writeSliderThumb(document, {
    id: "color-hue",
    maximum: DEGREES_PER_TURN,
    text: `${hue}°`,
    value: picked.hue,
  });

  writeSliderThumb(document, {
    id: "color-opacity",
    maximum: PERCENT_SLIDER_RANGE.maximum,
    text: `${String(opacity)} %`,
    value: opacity,
  });

  selectElement(document, "color-opacity").style.setProperty(
    "--picked-color",
    readShownColor(document, ids.color),
  );

  writeColorPlane(document, picked, model);
  writePickerValues(document, picked, model);
}
