import { DEFAULT_FONT, matchingFace } from "../font";

import type { FontFace, FontFamily, FontMetrics } from "../font";
import type { Text } from "../text";

/**
 * The faces of the family of a name among the fonts read.
 *
 * @kind domain
 * @param fonts - the font families whose metrics are known
 * @param name - a family's name
 * @returns its faces; none when no family has that name
 * @see docs/domain/shapes.md#text
 */
function familyFaces(fonts: readonly FontFamily[], name: string): readonly FontFace[] {
  return fonts.find((family) => family.name === name)?.faces ?? [];
}

/**
 * The metrics a text is measured with: those of the face of its font that matches its weight and
 * style (`matchingFace`), or, when its font has no metrics, of the default font's face that does
 * (ADR-0052).
 *
 * @kind domain
 * @param text - its font, weight and style
 * @param fonts - the font families whose metrics are known, the default font among them
 * @returns the face's metrics; undefined when neither its font nor the default font is among them
 * @see docs/domain/shapes.md#text
 */
export function textFontMetrics(text: Text, fonts: readonly FontFamily[]): FontMetrics | undefined {
  return (
    matchingFace(familyFaces(fonts, text.font), text) ??
    matchingFace(familyFaces(fonts, DEFAULT_FONT.name), text)
  )?.metrics;
}
