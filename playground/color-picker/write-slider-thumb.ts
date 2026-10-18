import { PERCENT, sliderPositionOf } from "../../src";
import { selectElement } from "../page";

/** A slider of the picker as shown: its value, its largest value and the value as read aloud. */
export type ShownSlider = {
  /** The slider's id; its thumb's is the same followed by `-thumb`. */
  readonly id: string;
  /** The value it holds. */
  readonly value: number;
  /** The value at its right end: 360 for the hue, 100 for the opacity. */
  readonly maximum: number;
  /** The value as a screen reader reads it, its unit after it. */
  readonly text: string;
};

/** A slider's length in percent of itself, for the thumb's place in CSS. */
const SLIDER_LENGTH_PERCENT = PERCENT;

/**
 * Shows a slider's value: its thumb where the value sits (`sliderPositionOf`), and the value and
 * its text for a screen reader (`REF-WAI-APG-SLIDER`: `aria-valuenow`, `aria-valuetext`).
 *
 * @kind procedure
 * @param document - page document
 * @param slider - the slider and its value
 * @see docs/backlog/stories/E15-F16-US-035-pick-a-color.md
 */
export function writeSliderThumb(document: Document, slider: ShownSlider): void {
  const element = selectElement(document, slider.id);
  const left = sliderPositionOf(slider.value, SLIDER_LENGTH_PERCENT, slider.maximum);

  element.setAttribute("aria-valuenow", String(slider.value));
  element.setAttribute("aria-valuetext", slider.text);
  selectElement(document, `${slider.id}-thumb`).style.left = `${String(left)}%`;
}
