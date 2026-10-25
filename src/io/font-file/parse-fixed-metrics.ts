import { isInIntegerRange } from "../../math";
import { tableToView } from "../font-tables";

import { parseCapHeight } from "./parse-cap-height";
import { parseEmbeddingPermissions } from "./parse-embedding-permissions";
import { parseHorizontalHeader } from "./parse-horizontal-header";
import { parseTextDecorations } from "./parse-text-decorations";
import { parseUnitsPerEm } from "./parse-units-per-em";

import type { FontMetrics } from "../../model";
import type { FontTables } from "../font-tables";

/** The metrics a font gives once for all its characters. */
export type FixedMetrics = Omit<FontMetrics, "advances" | "kerning" | "missingAdvance">;

/** The fewest units per em of a valid font. */
const MIN_UNITS_PER_EM = 16;

/** The most units per em of a valid font. */
const MAX_UNITS_PER_EM = 16_384;

/**
 * Reads what a font gives in its `OS/2` table for all its characters, apart from its strike-out:
 * its cap height and its embedding permissions.
 *
 * @kind format
 * @param os2 - the font's `OS/2` table, at least 30 bytes long
 * @param unitsPerEm - the font's units per em
 * @returns its cap height, font units, and its `fsType` flags
 */
function parseOs2Metrics(
  os2: Readonly<DataView>,
  unitsPerEm: number,
): Pick<FixedMetrics, "capHeight" | "embeddingPermissions"> {
  return {
    capHeight: parseCapHeight(os2, unitsPerEm),
    embeddingPermissions: parseEmbeddingPermissions(os2),
  };
}

/**
 * Reads the metrics a font gives once for all its characters: its units per em, its `hhea`
 * ascender, descender and line gap, its cap height, its underline, its strike-out and its
 * embedding permissions, each table long enough for what is read in it. A font cannot measure a
 * text when its units per em lie outside 16 to 16 384 (`REF-OT-HEAD`), or when `hhea` announces
 * no `hmtx` record, whose advance the glyphs past the records take (`REF-OT-HMTX`).
 *
 * @kind format
 * @param view - the font file
 * @param tables - where its tables sit
 * @returns those metrics, font units; undefined for units per em outside 16 to 16 384 or no
 * `hmtx` record
 */
export function parseFixedMetrics(
  view: Readonly<DataView>,
  tables: FontTables,
): FixedMetrics | undefined {
  const { numberOfHMetrics, ...lineMetrics } = parseHorizontalHeader(
    tableToView(view, tables.hhea),
  );
  const unitsPerEm = parseUnitsPerEm(tableToView(view, tables.head));

  if (numberOfHMetrics === 0 || !isInIntegerRange(unitsPerEm, MIN_UNITS_PER_EM, MAX_UNITS_PER_EM)) {
    return undefined;
  }

  const os2 = tableToView(view, tables.os2);

  return {
    ...parseTextDecorations(tableToView(view, tables.post), os2),
    ...parseOs2Metrics(os2, unitsPerEm),
    ...lineMetrics,
    unitsPerEm,
  };
}
