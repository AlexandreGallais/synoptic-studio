import type { Kerning } from "../kerning";

/** The metrics of a font, read once from its file (ADR-0052): everything a text needs to be measured and decorated, in font units. */
export type FontMetrics = {
  /** Font units per em, from 16 to 16 384 (`REF-OT-HEAD`). */
  readonly unitsPerEm: number;
  /** Typographic ascent of `hhea`, above the baseline (`REF-OT-HHEA`). */
  readonly ascender: number;
  /** Typographic descent of `hhea`, negative below the baseline (`REF-OT-HHEA`). */
  readonly descender: number;
  /** Typographic line gap of `hhea` (`REF-OT-HHEA`). */
  readonly lineGap: number;
  /** Height of the capitals above the baseline: `sCapHeight` of `OS/2`, or 0.66 em when the font gives none (`REF-OT-OS2`, `REF-CSS-INLINE-3`). */
  readonly capHeight: number;
  /** Advance of each character the font maps to a glyph other than 0, by code point (`REF-OT-CMAP`, `REF-OT-HMTX`). */
  readonly advances: Readonly<Record<number, number>>;
  /** Advance of glyph 0, `.notdef`, which draws every character the font does not map (`REF-OT-CMAP`). */
  readonly missingAdvance: number;
  /** Kerning between the characters of `advances` (`REF-OT-GPOS`, `REF-OT-KERN`). */
  readonly kerning: Kerning;
  /** Top of the underline, from the baseline, negative below it (`REF-OT-POST`). */
  readonly underlinePosition: number;
  /** Thickness of the underline (`REF-OT-POST`). */
  readonly underlineThickness: number;
  /** Top of the strike-out stroke, from the baseline (`REF-OT-OS2`). */
  readonly strikeoutPosition: number;
  /** Thickness of the strike-out stroke (`REF-OT-OS2`). */
  readonly strikeoutSize: number;
  /** `fsType` as the file holds it: 0 installable, 2 restricted, 4 preview and print, 8 editable (`REF-OT-OS2`). The raw field only: whether the file may be embedded is decided by `isEmbeddable`, with the `OS/2` version, and kept in `FontFile.isEmbeddable`. */
  readonly embeddingPermissions: number;
};
