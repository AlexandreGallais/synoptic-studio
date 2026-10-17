import { glyphAdvance, kerningOfCharacters } from "../../model";
import { parseCharacterMap } from "../character-map";
import { tableToView } from "../font-tables";
import { parseFontKerning } from "../kerning";

import { parseAdvanceWidths } from "./parse-advance-widths";
import { parseFixedMetrics } from "./parse-fixed-metrics";
import { parseHorizontalHeader } from "./parse-horizontal-header";

import type { CharacterGlyph, FontMetrics, KerningLookup } from "../../model";
import type { FontTables } from "../font-tables";

/**
 * Reads the advance of each character, and that of glyph 0.
 *
 * @kind format
 * @param view - the font file
 * @param tables - where its tables sit
 * @param glyphs - each character the font maps, and its glyph
 * @returns the advances by code point, and the advance of `.notdef`
 * @see REF-OT-HMTX
 */
function parseCharacterAdvances(
  view: Readonly<DataView>,
  tables: FontTables,
  glyphs: readonly CharacterGlyph[],
): Pick<FontMetrics, "advances" | "missingAdvance"> {
  const header = parseHorizontalHeader(tableToView(view, tables.hhea));
  const advances = parseAdvanceWidths(tableToView(view, tables.hmtx), header.numberOfHMetrics);

  return {
    advances: Object.fromEntries(
      glyphs.map(([codePoint, glyph]) => [codePoint, glyphAdvance(advances, glyph)]),
    ),
    missingAdvance: glyphAdvance(advances, 0),
  };
}

/**
 * Reads the kerning lookups of a font for the glyphs of some characters.
 *
 * @kind format
 * @param view - the font file
 * @param tables - where its tables sit
 * @param glyphs - each character the font maps, and its glyph
 * @returns its kerning lookups; undefined when a kerning table read passes its end
 * @see REF-OT-GPOS
 */
function parseKerningLookups(
  view: Readonly<DataView>,
  tables: FontTables,
  glyphs: readonly CharacterGlyph[],
): readonly KerningLookup[] | undefined {
  return parseFontKerning(
    tables.gpos === undefined ? undefined : tableToView(view, tables.gpos),
    tables.kern === undefined ? undefined : tableToView(view, tables.kern),
    new Set(glyphs.map(([, glyph]) => glyph)),
  );
}

/**
 * Reads the metrics of a font from its tables, for the characters it maps.
 *
 * @kind format
 * @param view - the font file
 * @param tables - where its tables sit
 * @param glyphs - each character the font maps among those asked, and its glyph
 * @returns its metrics; undefined when a kerning table read passes its end, or when the font cannot
 * measure a text (`parseFixedMetrics`)
 * @see REF-OT-OTFF
 */
function glyphsToMetrics(
  view: Readonly<DataView>,
  tables: FontTables,
  glyphs: readonly CharacterGlyph[],
): FontMetrics | undefined {
  const lookups = parseKerningLookups(view, tables, glyphs);
  const fixed = parseFixedMetrics(view, tables);

  return lookups === undefined || fixed === undefined
    ? undefined
    : {
        ...parseCharacterAdvances(view, tables, glyphs),
        ...fixed,
        kerning: kerningOfCharacters(lookups, glyphs),
      };
}

/**
 * Reads the metrics of a font from its tables, found once in its table directory
 * (`parseFontTables`), for the characters it maps among those asked.
 *
 * @kind format
 * @param view - the font file
 * @param tables - where its tables sit
 * @param characters - the code points whose advances and kerning are kept
 * @returns its metrics; undefined when the character map or a kerning table passes its end
 * @see REF-OT-CMAP
 */
export function fontTablesToMetrics(
  view: Readonly<DataView>,
  tables: FontTables,
  characters: readonly number[],
): FontMetrics | undefined {
  const mapped = parseCharacterMap(tableToView(view, tables.cmap), characters);

  return mapped === undefined
    ? undefined
    : glyphsToMetrics(
        view,
        tables,
        mapped.filter(([, glyph]) => glyph !== 0),
      );
}
