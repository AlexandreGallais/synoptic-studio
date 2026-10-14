import { glyphAdvance } from "../../model";

import { parseAdvanceWidths } from "./parse-advance-widths";
import { parseCharacterMap } from "./parse-character-map";
import { parseFontTables } from "./parse-font-tables";
import { parseHorizontalHeader } from "./parse-horizontal-header";
import { parseUnitsPerEm } from "./parse-units-per-em";

import type { FontTables } from "./font-tables";
import type { FontMetrics } from "../../model";

/**
 * Reads the advance of each character the font maps to a glyph other than 0, and that of glyph 0.
 *
 * @kind format
 * @param view - the font file
 * @param tables - where its tables sit
 * @param characters - the code points whose advances are kept
 * @returns the advances by code point, and the advance of `.notdef`
 * @see REF-OT-HMTX
 */
function parseCharacterAdvances(
  view: DataView,
  tables: FontTables,
  characters: readonly number[],
): Pick<FontMetrics, "advances" | "missingAdvance"> {
  const header = parseHorizontalHeader(view, tables.hhea);
  const advances = parseAdvanceWidths(view, tables.hmtx, header.numberOfHMetrics);

  return {
    advances: Object.fromEntries(
      parseCharacterMap(view, tables.cmap, characters)
        .filter(([, glyph]) => glyph !== 0)
        .map(([codePoint, glyph]) => [codePoint, glyphAdvance(advances, glyph)]),
    ),
    missingAdvance: glyphAdvance(advances, 0),
  };
}

/**
 * Reads the metrics of a font from its tables.
 *
 * @kind format
 * @param view - the font file
 * @param tables - where its tables sit
 * @param characters - the code points whose advances are kept
 * @returns its metrics
 * @see REF-OT-HHEA
 */
function tablesToMetrics(
  view: DataView,
  tables: FontTables,
  characters: readonly number[],
): FontMetrics {
  const { ascender, descender, lineGap } = parseHorizontalHeader(view, tables.hhea);

  return {
    ...parseCharacterAdvances(view, tables, characters),
    ascender,
    descender,
    lineGap,
    unitsPerEm: parseUnitsPerEm(view, tables.head),
  };
}

/**
 * Reads the metrics of a TrueType or OpenType font file once, so that a text is then measured
 * as a sum of numbers (ADR-0052): its units per em, its `hhea` vertical metrics, and the advance
 * of each character it maps among those asked.
 *
 * @kind format
 * @param file - the bytes of the font file
 * @param characters - the code points whose advances are kept
 * @returns its metrics; undefined for a file that is not a font, or lacks a table measuring needs
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
