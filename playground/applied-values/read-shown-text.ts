import { SHOWN_TEXTS } from "./shown-texts";
import { TYPED_NUMBER_FIELDS } from "./typed-number-fields";

/**
 * Reads the text of a field as the page reads it: the value applied, the text it showed before a
 * text was typed in it while that text is not applied (`TYPED_NUMBER_FIELDS`, `SHOWN_TEXTS`), so that a
 * redraw, a wheel's zoom, a font loaded or the canvas resized, draws nothing typed before Enter
 * (AUD-046); otherwise its text.
 *
 * @kind procedure
 * @param input - a field of the page
 * @returns the text applied
 * @see RULE-interaction-2-selector
 */
export function readShownText(input: HTMLInputElement): string {
  return TYPED_NUMBER_FIELDS.has(input) ? (SHOWN_TEXTS.get(input) ?? input.value) : input.value;
}
