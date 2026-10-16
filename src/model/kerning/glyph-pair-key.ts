import { rowMajorOffset } from "../../math";

import { GLYPH_INDEX_COUNT } from "./glyph-index-count";

/**
 * The key of a pair of glyphs: the unsigned 32-bit integer whose high-order word is the first
 * glyph and whose low-order word is the second, as a `kern` table orders its pairs: the offset of
 * the pair in a table of 65 536 × 65 536 pairs laid out row by row (`rowMajorOffset`).
 *
 * Formula: first × 65 536 + second.
 *
 * @kind domain
 * @param first - glyph index of the first character, from 0 to 65 535
 * @param second - glyph index of the second character, from 0 to 65 535
 * @returns the pair's key, from 0 to 2³² − 1
 * @see REF-OT-KERN
 */
export function glyphPairKey(first: number, second: number): number {
  return rowMajorOffset(first, second, GLYPH_INDEX_COUNT);
}
