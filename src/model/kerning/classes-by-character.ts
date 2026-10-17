import type { CharacterGlyph } from "../font";

/**
 * The class of each character that kerns, by code point; a character in class 0, or past the
 * classes given, is left out, as it kerns with nothing.
 *
 * @kind domain
 * @param glyphs - the characters, in the order of the classes
 * @param classes - the class of each character
 * @returns the classes other than 0, by code point
 * @see DERIV-kerning-classes
 */
export function classesByCharacter(
  glyphs: readonly CharacterGlyph[],
  classes: readonly number[],
): Readonly<Record<number, number>> {
  return Object.fromEntries(
    glyphs
      .map(([codePoint], index): readonly [number, number] => [codePoint, classes[index] ?? 0])
      .filter(([, group]) => group !== 0),
  );
}
