import { DEFAULT_FONT, isSameFamilyName, matchingFace } from "../font";

import type { FontFace, FontFamily } from "../font";
import type { NamedFontFace } from "../font-import";
import type { Text } from "../text";

/**
 * The face of a family that matches a text's weight and style, with the family's name.
 *
 * @kind domain
 * @param family - a family read
 * @param style - the text's weight and style
 * @returns the face and its family's name; undefined for a family without faces
 * @see REF-CSS-FONTS-4
 */
function namedMatchingFace(
  family: FontFamily,
  style: Pick<FontFace, "isItalic" | "weight">,
): NamedFontFace | undefined {
  const face = matchingFace(family.faces, style);

  return face === undefined ? undefined : { face, family: family.name };
}

/**
 * The face of the family of a name among the fonts read that matches a text's weight and style,
 * the name matched without regard to case (`isSameFamilyName`).
 *
 * @kind domain
 * @param fonts - the font families whose metrics are known
 * @param name - a family's name
 * @param text - its weight and style
 * @returns the face and its family's name, as read; undefined when no family of that name has a face
 * @see docs/domain/shapes.md#text
 */
function familyFace(
  fonts: readonly FontFamily[],
  name: string,
  text: Text,
): NamedFontFace | undefined {
  const family = fonts.find((kept) => isSameFamilyName(name, kept.name));

  return family === undefined ? undefined : namedMatchingFace(family, text);
}

/**
 * The face a text is measured with, and its family's name as read: the face that matches its
 * weight and style (`matchingFace`) in its font; when its font has no metrics, in its fallback
 * family; then in the default font (ADR-0055, decision 1). A document embeds it (ADR-0053).
 *
 * @kind domain
 * @param text - its font, fallback, weight and style
 * @param fonts - the font families whose metrics are known, the default font among them
 * @returns the face and its family's name; undefined when none of the three families is among them
 * @see docs/domain/shapes.md#text
 */
export function textFontFace(text: Text, fonts: readonly FontFamily[]): NamedFontFace | undefined {
  return (
    familyFace(fonts, text.font, text) ??
    familyFace(fonts, text.fallback, text) ??
    familyFace(fonts, DEFAULT_FONT.name, text)
  );
}
