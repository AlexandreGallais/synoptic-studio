import { DEFAULT_FONT, isSameFamilyName, matchingFace } from "../font";

import type { FontFace, FontFamily, FontMetrics } from "../font";
import type { Text } from "../text";

/**
 * The faces of the family of a name among the fonts read, the name matched without regard to
 * case (`isSameFamilyName`).
 *
 * @kind domain
 * @param fonts - the font families whose metrics are known
 * @param name - a family's name
 * @returns its faces; none when no family has that name
 * @see docs/domain/shapes.md#text
 */
function familyFaces(fonts: readonly FontFamily[], name: string): readonly FontFace[] {
  return fonts.find((family) => isSameFamilyName(name, family.name))?.faces ?? [];
}

/**
 * The metrics a text is measured with: those of the face that matches its weight and style
 * (`matchingFace`) in its font; when its font has no metrics, in its fallback family; then in the
 * default font (ADR-0055, decision 1).
 *
 * @kind domain
 * @param text - its font, fallback, weight and style
 * @param fonts - the font families whose metrics are known, the default font among them
 * @returns the face's metrics; undefined when none of the three families is among them
 * @see docs/domain/shapes.md#text
 */
export function textFontMetrics(text: Text, fonts: readonly FontFamily[]): FontMetrics | undefined {
  return (
    matchingFace(familyFaces(fonts, text.font), text) ??
    matchingFace(familyFaces(fonts, text.fallback), text) ??
    matchingFace(familyFaces(fonts, DEFAULT_FONT.name), text)
  )?.metrics;
}
