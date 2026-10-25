import type { ColorField } from "./color-field";

/**
 * Reads the fields a swatch stands for, from its `data-color`, `data-opacity`, `data-message`
 * and, for a color that follows the text's, `data-follows` attributes.
 *
 * @kind procedure
 * @param swatch - the button painted with the color, which opens the picker
 * @returns the ids of its fields; empty for an element that is no swatch
 * @see US-035
 */
export function readColorField(swatch: HTMLElement): ColorField {
  const { color = "", follows = "", message = "", opacity = "" } = swatch.dataset;

  return { color, follows, message, opacity };
}
