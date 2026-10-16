import { maximum, transpose } from "../../math";

import { classesByCharacter } from "./classes-by-character";
import { glyphPairKerning } from "./glyph-pair-kerning";
import { kerningClasses } from "./kerning-classes";

import type { CharacterGlyph } from "../font";
import type { Kerning } from "./kerning";
import type { KerningLookup } from "./kerning-lookup";
import type { KerningRow } from "./kerning-row";

/**
 * The kerning of each pair of classes, read on a pair of characters of those classes; class 0
 * kerns with nothing, as its characters do.
 *
 * @kind domain
 * @param rows - the kerning of each character with every other one
 * @param leftClasses - the class of each row
 * @param rightClasses - the class of each column
 * @returns the kerning by left class, then right class
 * @see DERIV-kerning-classes
 */
function valuesByClasses(
  rows: readonly KerningRow[],
  leftClasses: readonly number[],
  rightClasses: readonly number[],
): readonly KerningRow[] {
  const columns = Array.from({ length: maximum(rightClasses, 0) + 1 }, (unused, group) =>
    rightClasses.indexOf(group),
  );

  return Array.from({ length: maximum(leftClasses, 0) + 1 }, (unused, group) => {
    const row = rows[leftClasses.indexOf(group)];

    return columns.map((column) => row?.[column] ?? 0);
  });
}

/**
 * The kerning between characters, by classes: the kerning of each pair of characters, read in
 * the font's lookups, then the characters that kern alike grouped into a class, first as the left
 * character of a pair, then as the right one.
 *
 * @kind domain
 * @param lookups - the font's kerning lookups, in lookup list order
 * @param glyphs - each character and its glyph
 * @returns the kerning between those characters
 * @see DERIV-kerning-classes
 */
export function kerningOfCharacters(
  lookups: readonly KerningLookup[],
  glyphs: readonly CharacterGlyph[],
): Kerning {
  const rows = glyphs.map(([, first]) =>
    glyphs.map(([, second]) => glyphPairKerning(lookups, first, second)),
  );
  const leftClasses = kerningClasses(rows);
  const rightClasses = kerningClasses(transpose(rows));

  return {
    leftClasses: classesByCharacter(glyphs, leftClasses),
    rightClasses: classesByCharacter(glyphs, rightClasses),
    values: valuesByClasses(rows, leftClasses, rightClasses),
  };
}
