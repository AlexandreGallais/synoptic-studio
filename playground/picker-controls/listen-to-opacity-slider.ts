import {
  PERCENT_SLIDER_RANGE,
  nearestInteger,
  sliderValueAt,
  valueAfterSliderKey,
} from "../../src";
import { applyPickedOpacity } from "../color-picker";
import { selectElement } from "../page";

import { listenToDrag } from "./listen-to-drag";

/**
 * Listens to the opacity slider: a click or a drag sets the opacity where it is, to the nearest
 * percent (`sliderValueAt`, `nearestInteger`); the keys of a WAI-ARIA slider change it
 * (`valueAfterSliderKey`).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 */
export function listenToOpacitySlider(document: Document, update: () => void): void {
  const slider = selectElement(document, "color-opacity");
  const { maximum } = PERCENT_SLIDER_RANGE;

  listenToDrag(slider, ({ point, size }) => {
    applyPickedOpacity(
      document,
      nearestInteger(sliderValueAt(point.x, size.width, maximum)),
      update,
    );
  });

  slider.addEventListener("keydown", (event) => {
    const opacity = Number(slider.getAttribute("aria-valuenow"));
    const after = valueAfterSliderKey(event.key, opacity, PERCENT_SLIDER_RANGE);

    if (after === undefined) {
      return;
    }

    event.preventDefault();
    applyPickedOpacity(document, after, update);
  });
}
