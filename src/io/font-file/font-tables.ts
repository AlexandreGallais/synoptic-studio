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
};
