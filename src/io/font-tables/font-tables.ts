import type { FontTableRecord } from "./font-table-record";

/** The tables a font file needs to be measured, each found in its table directory (`REF-OT-OTFF`). */
export type FontTables = {
  /** Character to glyph mapping (`REF-OT-CMAP`). */
  readonly cmap: FontTableRecord;
  /** Font header: units per em (`REF-OT-HEAD`). */
  readonly head: FontTableRecord;
  /** Horizontal header: vertical metrics and `numberOfHMetrics` (`REF-OT-HHEA`). */
  readonly hhea: FontTableRecord;
  /** Horizontal metrics: the advance of each glyph (`REF-OT-HMTX`). */
  readonly hmtx: FontTableRecord;
  /** PostScript table: the underline (`REF-OT-POST`). */
  readonly post: FontTableRecord;
  /** OS/2 and Windows metrics: the strike-out and the embedding permissions (`REF-OT-OS2`). */
  readonly os2: FontTableRecord;
  /** Glyph positioning: kerning by pairs and classes, if the font has it (`REF-OT-GPOS`). */
  readonly gpos: FontTableRecord | undefined;
  /** Naming table: the family and version names, if the font has it (`REF-OT-NAME`). */
  readonly name: FontTableRecord | undefined;
  /** Kerning pairs of TrueType fonts, if the font has them (`REF-OT-KERN`). */
  readonly kern: FontTableRecord | undefined;
};
