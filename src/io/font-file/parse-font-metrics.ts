import { parseFontTables } from "../font-tables";

import { fontTablesToMetrics } from "./font-tables-to-metrics";

import type { FontMetrics } from "../../model";

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
 * holds a table too short for what it points to, has units per em outside 16 to 16 384 or no
 * advance record
 * @see REF-OT-OTFF
 */
export function parseFontMetrics(
  file: Readonly<Uint8Array>,
  characters: readonly number[],
): FontMetrics | undefined {
  const view = new DataView(file.buffer, file.byteOffset, file.byteLength);
  const tables = parseFontTables(view, 0);

  return tables === undefined ? undefined : fontTablesToMetrics(view, tables, characters);
}
