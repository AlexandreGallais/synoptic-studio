import { textFontFace } from "./text-font-face";

import type { FontFamily, FontMetrics } from "../font";
import type { Text } from "../text";

/**
 * The metrics a text is measured with: those of the face that matches its weight and style
 * (`matchingFace`) in its font; when its font has no metrics, in its fallback family; then in the
 * default font (ADR-0055, decision 1), as `textFontFace` finds it.
 *
 * @kind domain
 * @param text - its font, fallback, weight and style
 * @param fonts - the font families whose metrics are known, the default font among them
 * @returns the face's metrics; undefined when none of the three families is among them
 * @see RULE-shapes-text
 */
export function textFontMetrics(text: Text, fonts: readonly FontFamily[]): FontMetrics | undefined {
  return textFontFace(text, fonts)?.face.metrics;
}
