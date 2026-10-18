/**
 * The index of a value in a range of consecutive values whose indexes increase one by one from
 * the range's first: a Coverage range's records "begin with zero (0) and increase sequentially"
 * from its `startCoverageIndex` (`REF-OT-LAYOUT-COMMON`, Coverage format 2), as the glyphs of a
 * `cmap` group from its `startGlyphID` (`REF-OT-CMAP`, format 12). Glyph 42 in a range starting
 * at glyph 40 with index 7 has index 9.
 *
 * Formula: startIndex + (value − rangeStart).
 *
 * @kind math
 * @param value - a value of the range
 * @param rangeStart - the range's first value
 * @param startIndex - the index of that first value
 * @returns the value's index
 * @see REF-OT-LAYOUT-COMMON
 */
export function indexInRange(value: number, rangeStart: number, startIndex: number): number {
  return startIndex + (value - rangeStart);
}
