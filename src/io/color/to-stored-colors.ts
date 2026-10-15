import { parseHexColor } from "./parse-hex-color";

import type { ColorFill, Text } from "../../model";

/**
 * Reads one color and its opacity into their stored form: a color `parseHexColor` reads becomes
 * `#` and six lower-case digits, its alpha, if any, giving the opacity; a color it refuses is
 * left as it is, for the model's check to refuse.
 *
 * @kind format
 * @param color - the color as it was typed or loaded
 * @param opacity - its opacity as it was typed or loaded
 * @returns the color and the opacity to keep
 * @see REF-CSS-COLOR-4
 */
function parseStoredColor(color: string, opacity: number): ColorFill {
  const parsed = parseHexColor(color);

  return parsed === undefined
    ? { color, opacity }
    : { color: parsed.color, opacity: parsed.opacity ?? opacity };
}

/**
 * A text with its color, its background color and its underline's and strike-through's colors
 * (US-041) in their stored form, `#` and six lower-case digits, as every reader of a color does before the model's check (ADR-0056): a color of F05 or
 * F14 typed or loaded in upper case, or in another hexadecimal form, is turned to it, its alpha,
 * if any, setting its opacity; a color that is no hexadecimal is left as it is, so that
 * `isValidText` still refuses it.
 *
 * @kind format
 * @param text - the text as it was typed or loaded
 * @returns the same text, its colors in their stored form
 * @see REF-CSS-COLOR-4
 */
export function toStoredColors(text: Text): Text {
  const color = parseStoredColor(text.color, text.opacity);
  const background = parseStoredColor(text.backgroundColor, text.backgroundOpacity);
  const underline = parseStoredColor(text.underlineColor, text.underlineOpacity);
  const strikeThrough = parseStoredColor(text.strikeThroughColor, text.strikeThroughOpacity);

  return {
    ...text,
    backgroundColor: background.color,
    backgroundOpacity: background.opacity,
    color: color.color,
    opacity: color.opacity,
    strikeThroughColor: strikeThrough.color,
    strikeThroughOpacity: strikeThrough.opacity,
    underlineColor: underline.color,
    underlineOpacity: underline.opacity,
  };
}
