import { parseSegmentMappingGlyphs } from "./parse-segment-mapping-glyphs";
import { parseSegmentedCoverageGlyphs } from "./parse-segmented-coverage-glyphs";
import { parseUnicodeSubtables } from "./parse-unicode-subtables";

import type { CharacterGlyph } from "./character-glyph";
import type { FontTableRecord } from "./font-table-record";

/** `cmap` subtable format 4, segment mapping to delta values: the Basic Multilingual Plane. */
const SEGMENT_MAPPING = 4;

/** `cmap` subtable format 12, segmented coverage: every Unicode plane. */
const SEGMENTED_COVERAGE = 12;

/**
 * Reads the glyph of each character from a font's `cmap` table: from its Unicode subtable of
 * format 12 if it has one, else of format 4. Another format is not read: its characters take
 * glyph 0.
 *
 * @kind format
 * @param view - the font file
 * @param cmap - where its `cmap` table sits
 * @param characters - the code points to map
 * @returns each character and its glyph, 0 for a character the font does not map
 * @see REF-OT-CMAP
 */
export function parseCharacterMap(
  view: DataView,
  cmap: FontTableRecord,
  characters: readonly number[],
): readonly CharacterGlyph[] {
  const subtables = parseUnicodeSubtables(view, cmap);
  const coverage = subtables.find((subtable) => view.getUint16(subtable) === SEGMENTED_COVERAGE);
  const segments = subtables.find((subtable) => view.getUint16(subtable) === SEGMENT_MAPPING);

  if (coverage !== undefined) {
    return parseSegmentedCoverageGlyphs(view, coverage, characters);
  }

  return segments === undefined
    ? characters.map((codePoint): CharacterGlyph => [codePoint, 0])
    : parseSegmentMappingGlyphs(view, segments, characters);
}
