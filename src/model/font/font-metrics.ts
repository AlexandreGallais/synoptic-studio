/** The metrics of a font, read once from its file (ADR-0052): everything a text needs to be measured, in font units. */
export type FontMetrics = {
  /** Font units per em, from 16 to 16 384 (`REF-OT-HEAD`). */
  readonly unitsPerEm: number;
  /** Typographic ascent of `hhea`, above the baseline (`REF-OT-HHEA`). */
  readonly ascender: number;
  /** Typographic descent of `hhea`, negative below the baseline (`REF-OT-HHEA`). */
  readonly descender: number;
  /** Typographic line gap of `hhea` (`REF-OT-HHEA`). */
  readonly lineGap: number;
  /** Advance of each character the font maps to a glyph other than 0, by code point (`REF-OT-CMAP`, `REF-OT-HMTX`). */
  readonly advances: Readonly<Record<number, number>>;
  /** Advance of glyph 0, `.notdef`, which draws every character the font does not map (`REF-OT-CMAP`). */
  readonly missingAdvance: number;
};
