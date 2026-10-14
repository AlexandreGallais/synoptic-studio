import { parseGposKerning } from "./parse-gpos-kerning";
import { parseKernTable } from "./parse-kern-table";

import type { KerningLookup } from "../../model";

/**
 * Reads the kerning of a font: the lookups of the `kern` feature of its `GPOS` table, or, when it
 * has none, the subtables of its `kern` table, as a browser does for a font whose `GPOS` table
 * lacks the feature.
 *
 * @kind format
 * @param view - the font file
 * @param gpos - where its `GPOS` table starts in the file, if it has one
 * @param kern - where its `kern` table starts in the file, if it has one
 * @returns its kerning lookups, in the order they apply
 * @see REF-CSS-FONTS-4
 */
export function parseFontKerning(
  view: DataView,
  gpos: number | undefined,
  kern: number | undefined,
): readonly KerningLookup[] {
  const positioning = gpos === undefined ? [] : parseGposKerning(view, gpos);

  if (positioning.length > 0) {
    return positioning;
  }

  return kern === undefined ? [] : parseKernTable(view, kern);
}
