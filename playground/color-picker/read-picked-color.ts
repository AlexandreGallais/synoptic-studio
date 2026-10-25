import { selectElement } from "../page";

import type { PickedColor } from "../../src";

/**
 * Reads the values the open picker holds, kept on it while it is open (`writePickedColor`): its
 * hue, kept even for a grey, its saturation and its lightness or brightness.
 *
 * @kind procedure
 * @param document - page document
 * @returns the values; `NaN` for a picker never opened
 * @see US-035
 */
export function readPickedColor(document: Document): PickedColor {
  const { hue, level, saturation } = selectElement(document, "color-picker").dataset;

  return { hue: Number(hue), level: Number(level), saturation: Number(saturation) };
}
