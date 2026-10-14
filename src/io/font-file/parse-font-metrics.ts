import { glyphAdvance, kerningOfCharacters } from "../../model";
import { parseCharacterMap } from "../character-map";
import { parseFontKerning } from "../kerning";

import { parseAdvanceWidths } from "./parse-advance-widths";
import { parseFixedMetrics } from "./parse-fixed-metrics";
import { parseFontTables } from "./parse-font-tables";
import { parseHorizontalHeader } from "./parse-horizontal-header";
import { tableToView } from "./table-to-view";

import type { FontTables } from "./font-tables";
import type { CharacterGlyph, FontMetrics, KerningLookup } from "../../model";

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
  view: DataView,
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
  view: DataView,
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
 * @returns its metrics; undefined when a kerning table read passes its end
 * @see REF-OT-OTFF
 */
function glyphsToMetrics(
  view: DataView,
  tables: FontTables,
  glyphs: readonly CharacterGlyph[],
): FontMetrics | undefined {
  const lookups = parseKerningLookups(view, tables, glyphs);

  return lookups === undefined
    ? undefined
    : {
        ...parseCharacterAdvances(view, tables, glyphs),
        ...parseFixedMetrics(view, tables),
        kerning: kerningOfCharacters(lookups, glyphs),
      };
}

/**
 * Reads the metrics of a font from its tables, for the characters it maps among those asked.
 *
 * @kind format
 * @param view - the font file
 * @param tables - where its tables sit
 * @param characters - the code points whose advances and kerning are kept
 * @returns its metrics; undefined when the character map or a kerning table passes its end
 * @see REF-OT-CMAP
 */
function tablesToMetrics(
  view: DataView,
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

/**
 * Reads the metrics of a TrueType or OpenType font file once, so that a text is then measured
 * as a sum of numbers (ADR-0052): its units per em, its `hhea` vertical metrics, its underline and
 * strike-out, its embedding permissions, and the advance and kerning of each character it maps
 * among those asked. Every read is checked against its table: a damaged file gives no metrics,
 * never an error.
 *
 * @kind format
 * @param file - the bytes of the font file
 * @param characters - the code points whose advances and kerning are kept
 * @returns its metrics; undefined for a file that is not a font, lacks a table measuring needs,
 * or holds a table too short for what it points to
 * @see REF-OT-OTFF
 */
export function parseFontMetrics(
  file: Readonly<Uint8Array>,
  characters: readonly number[],
): FontMetrics | undefined {
  const view = new DataView(file.buffer, file.byteOffset, file.byteLength);
  const tables = parseFontTables(view);

  return tables === undefined ? undefined : tablesToMetrics(view, tables, characters);
}
