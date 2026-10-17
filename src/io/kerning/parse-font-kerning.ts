import { parseGposKerning } from "./parse-gpos-kerning";
import { parseKernTable } from "./parse-kern-table";

import type { KerningLookup } from "../../model";

/**
 * Reads the kerning of a font for some glyphs: the lookups of the `kern` feature of its `GPOS`
 * table, or, when it has none, the subtables of its `kern` table, as CSS Fonts 4 allows a reader to
 * do for a font whose `GPOS` table lacks the feature.
 *
 * @kind format
 * @param gpos - its `GPOS` table, at least its 10-byte header, if it has one
 * @param kern - its `kern` table, at least its 4-byte header, if it has one
 * @param glyphs - the glyphs asked, first and second
 * @returns its kerning lookups, in the order they apply; undefined when a table read passes its
 * end
 * @see REF-CSS-FONTS-4
 */
export function parseFontKerning(
  gpos: Readonly<DataView> | undefined,
  kern: Readonly<DataView> | undefined,
  glyphs: ReadonlySet<number>,
): readonly KerningLookup[] | undefined {
  const positioning = gpos === undefined ? [] : parseGposKerning(gpos, glyphs);

  if (positioning?.length !== 0) {
    return positioning;
  }

  return kern === undefined ? [] : parseKernTable(kern, glyphs);
}
