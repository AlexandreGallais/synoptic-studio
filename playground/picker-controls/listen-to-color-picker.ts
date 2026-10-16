import { listenToColorFields } from "./listen-to-color-fields";
import { listenToColorPlane } from "./listen-to-color-plane";
import { listenToHueSlider } from "./listen-to-hue-slider";
import { listenToOpacitySlider } from "./listen-to-opacity-slider";
import { listenToPickerClosing } from "./listen-to-picker-closing";
import { listenToPickerValues } from "./listen-to-picker-values";

/**
 * Listens to each color's fields and to the color picker: its opening and closing, its hue and opacity sliders, its plane
 * and its values (`docs/domain/interaction.md` §5).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E15-F16-US-035-pick-a-color.md
 */
export function listenToColorPicker(document: Document, update: () => void): void {
  listenToColorFields(document, update);
  listenToPickerClosing(document);
  listenToHueSlider(document, update);
  listenToOpacitySlider(document, update);
  listenToColorPlane(document, update);
  listenToPickerValues(document, update);
}
