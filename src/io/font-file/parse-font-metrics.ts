import { glyphAdvance, kerningOfCharacters } from "../../model";
import { parseFontKerning } from "../kerning";

import { parseAdvanceWidths } from "./parse-advance-widths";
import { parseCharacterMap } from "./parse-character-map";
import { parseEmbeddingPermissions } from "./parse-embedding-permissions";
import { parseFontTables } from "./parse-font-tables";
import { parseHorizontalHeader } from "./parse-horizontal-header";
import { parseTextDecorations } from "./parse-text-decorations";
import { parseUnitsPerEm } from "./parse-units-per-em";

import type { FontTables } from "./font-tables";
import type { CharacterGlyph, FontMetrics } from "../../model";

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
  const header = parseHorizontalHeader(view, tables.hhea);
  const advances = parseAdvanceWidths(view, tables.hmtx, header.numberOfHMetrics);

  return {
    advances: Object.fromEntries(
      glyphs.map(([codePoint, glyph]) => [codePoint, glyphAdvance(advances, glyph)]),
    ),
    missingAdvance: glyphAdvance(advances, 0),
  };
}

/**
 * Reads the units per em of a font, and its `hhea` ascender, descender and line gap.
 *
 * @kind format
 * @param view - the font file
 * @param tables - where its tables sit
 * @returns the four metrics, font units
 * @see REF-OT-HHEA
 */
function parseVerticalMetrics(
  view: DataView,
  tables: FontTables,
): Pick<FontMetrics, "ascender" | "descender" | "lineGap" | "unitsPerEm"> {
  const { ascender, descender, lineGap } = parseHorizontalHeader(view, tables.hhea);

  return { ascender, descender, lineGap, unitsPerEm: parseUnitsPerEm(view, tables.head) };
}

/**
 * Reads the metrics of a font from its tables, for the characters it maps among those asked.
 *
 * @kind format
 * @param view - the font file
 * @param tables - where its tables sit
 * @param characters - the code points whose advances and kerning are kept
 * @returns its metrics
 * @see REF-OT-OTFF
 */
function tablesToMetrics(
  view: DataView,
  tables: FontTables,
  characters: readonly number[],
): FontMetrics {
  const glyphs = parseCharacterMap(view, tables.cmap, characters).filter(
    ([, glyph]) => glyph !== 0,
  );
  const lookups = parseFontKerning(view, tables.gpos?.offset, tables.kern?.offset);

  return {
    ...parseCharacterAdvances(view, tables, glyphs),
    ...parseVerticalMetrics(view, tables),
    ...parseTextDecorations(view, tables),
    embeddingPermissions: parseEmbeddingPermissions(view, tables.os2),
    kerning: kerningOfCharacters(lookups, glyphs),
  };
}

/**
 * Reads the metrics of a TrueType or OpenType font file once, so that a text is then measured
 * as a sum of numbers (ADR-0052): its units per em, its `hhea` vertical metrics, its underline and
 * strike-out, its embedding permissions, and the advance and kerning of each character it maps
 * among those asked.
 *
 * @kind format
 * @param file - the bytes of the font file
 * @param characters - the code points whose advances and kerning are kept
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
