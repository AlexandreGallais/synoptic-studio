import {
  DEGREES_PER_TURN,
  HUE_SLIDER_RANGE,
  sliderValueAt,
  toShownHue,
  valueAfterSliderKey,
} from "editor";

import { applyPickedColor, readPickedColor } from "../color-picker";
import { selectElement } from "../page";

import { listenToDrag } from "./listen-to-drag";

/**
 * Listens to the hue slider: a click or a drag sets the hue where it is, 360° at the right end
 * shown 0, a whole number (`sliderValueAt`, `toShownHue`); the keys of a WAI-ARIA slider change it
 * (`valueAfterSliderKey`); the saturation and lightness or brightness stay.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 */
export function listenToHueSlider(document: Document, update: () => void): void {
  const slider = selectElement(document, "color-hue");

  listenToDrag(slider, ({ point, size }) => {
    const hue = toShownHue(sliderValueAt(point.x, size.width, DEGREES_PER_TURN));

    applyPickedColor(document, { ...readPickedColor(document), hue }, update);
  });

  slider.addEventListener("keydown", (event) => {
    const picked = readPickedColor(document);
    const hue = valueAfterSliderKey(event.key, picked.hue, HUE_SLIDER_RANGE);

    if (hue === undefined) {
      return;
    }

    event.preventDefault();
    applyPickedColor(document, { ...picked, hue: toShownHue(hue) }, update);
  });
}
