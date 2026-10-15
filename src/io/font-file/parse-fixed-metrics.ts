import { isInIntegerRange } from "../../math";

import { parseEmbeddingPermissions } from "./parse-embedding-permissions";
import { parseHorizontalHeader } from "./parse-horizontal-header";
import { parseTextDecorations } from "./parse-text-decorations";
import { parseUnitsPerEm } from "./parse-units-per-em";
import { tableToView } from "./table-to-view";

import type { FontTables } from "./font-tables";
import type { FontMetrics } from "../../model";

/** The metrics a font gives once for all its characters. */
export type FixedMetrics = Omit<FontMetrics, "advances" | "kerning" | "missingAdvance">;

/** The fewest units per em of a valid font. */
const MIN_UNITS_PER_EM = 16;

/** The most units per em of a valid font. */
const MAX_UNITS_PER_EM = 16_384;

/**
 * Reads the metrics a font gives once for all its characters: its units per em, its `hhea`
 * ascender, descender and line gap, its underline, its strike-out and its embedding permissions,
 * each table long enough for what is read in it. A font cannot measure a text when its units per
 * em lie outside 16 to 16 384 (`REF-OT-HEAD`), or when `hhea` announces no `hmtx` record, whose
 * advance the glyphs past the records take (`REF-OT-HMTX`).
 *
 * @kind format
 * @param view - the font file
 * @param tables - where its tables sit
 * @returns those metrics, font units; undefined for units per em outside 16 to 16 384 or no
 * `hmtx` record
 * @see REF-OT-HHEA
 */
export function parseFixedMetrics(view: DataView, tables: FontTables): FixedMetrics | undefined {
  const header = parseHorizontalHeader(tableToView(view, tables.hhea));
  const { ascender, descender, lineGap } = header;
  const unitsPerEm = parseUnitsPerEm(tableToView(view, tables.head));

  if (
    header.numberOfHMetrics === 0 ||
    !isInIntegerRange(unitsPerEm, MIN_UNITS_PER_EM, MAX_UNITS_PER_EM)
  ) {
    return undefined;
  }

  const os2 = tableToView(view, tables.os2);

  return {
    ...parseTextDecorations(tableToView(view, tables.post), os2),
    ascender,
    descender,
    embeddingPermissions: parseEmbeddingPermissions(os2),
    lineGap,
    unitsPerEm,
  };
}
