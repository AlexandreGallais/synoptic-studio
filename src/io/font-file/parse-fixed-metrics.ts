import { parseEmbeddingPermissions } from "./parse-embedding-permissions";
import { parseHorizontalHeader } from "./parse-horizontal-header";
import { parseTextDecorations } from "./parse-text-decorations";
import { parseUnitsPerEm } from "./parse-units-per-em";
import { tableToView } from "./table-to-view";

import type { FontTables } from "./font-tables";
import type { FontMetrics } from "../../model";

/** The metrics a font gives once for all its characters. */
export type FixedMetrics = Omit<FontMetrics, "advances" | "kerning" | "missingAdvance">;

/**
 * Reads the metrics a font gives once for all its characters: its units per em, its `hhea`
 * ascender, descender and line gap, its underline, its strike-out and its embedding permissions,
 * each table long enough for what is read in it.
 *
 * @kind format
 * @param view - the font file
 * @param tables - where its tables sit
 * @returns those metrics, font units
 * @see REF-OT-HHEA
 */
export function parseFixedMetrics(view: DataView, tables: FontTables): FixedMetrics {
  const { ascender, descender, lineGap } = parseHorizontalHeader(tableToView(view, tables.hhea));
  const os2 = tableToView(view, tables.os2);

  return {
    ...parseTextDecorations(tableToView(view, tables.post), os2),
    ascender,
    descender,
    embeddingPermissions: parseEmbeddingPermissions(os2),
    lineGap,
    unitsPerEm: parseUnitsPerEm(tableToView(view, tables.head)),
  };
}
