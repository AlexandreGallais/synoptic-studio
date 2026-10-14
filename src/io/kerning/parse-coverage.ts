import { isSpanWithin } from "../../math";

import { parseGlyphRange } from "./parse-glyph-range";

/** Coverage format 1: a list of glyphs. */
const GLYPH_LIST = 1;

/** Coverage format 2: ranges of glyphs. */
const GLYPH_RANGES = 2;

/** Size of a Coverage table's header: the format and the count. */
const HEADER_SIZE = 4;

/** Size of one glyph of a list. */
const GLYPH_SIZE = 2;

/** Size of one range record: start glyph, end glyph, start coverage index. */
const RANGE_RECORD_SIZE = 6;

/** Offset of a range's start coverage index in its record. */
const START_INDEX_FIELD = 4;

/** The coverage index of each glyph covered, by glyph. */
type CoverageIndexes = Readonly<Record<number, number>>;

/**
 * Reads a glyph list (format 1): the glyph at each coverage index, the glyphs kept those asked.
 *
 * @kind format
 * @param table - the layout table holding the Coverage table
 * @param coverage - where the Coverage table starts in it
 * @param glyphs - the glyphs to look for
 * @returns the coverage index of each glyph asked that is covered; undefined when the list
 * passes the table's end
 * @see REF-OT-LAYOUT-COMMON
 */
function parseGlyphList(
  table: DataView,
  coverage: number,
  glyphs: ReadonlySet<number>,
): CoverageIndexes | undefined {
  const count = table.getUint16(coverage + 2);

  return isSpanWithin(coverage, HEADER_SIZE + GLYPH_SIZE * count, table.byteLength)
    ? Object.fromEntries(
        Array.from({ length: count }, (unused, index): readonly [number, number] => [
          table.getUint16(coverage + HEADER_SIZE + GLYPH_SIZE * index),
          index,
        ]).filter(([glyph]) => glyphs.has(glyph)),
      )
    : undefined;
}

/**
 * Reads glyph ranges (format 2): each glyph asked is looked up in the ranges, whose coverage
 * indexes run on from each range's start index; no range is spread out glyph by glyph.
 *
 * @kind format
 * @param table - the layout table holding the Coverage table
 * @param coverage - where the Coverage table starts in it
 * @param glyphs - the glyphs to look for
 * @returns the coverage index of each glyph asked that is covered; undefined when the ranges pass
 * the table's end
 * @see REF-OT-LAYOUT-COMMON
 */
function parseGlyphRanges(
  table: DataView,
  coverage: number,
  glyphs: ReadonlySet<number>,
): CoverageIndexes | undefined {
  const count = table.getUint16(coverage + 2);
  const ranges = Array.from(
    { length: count },
    (unused, index) => coverage + HEADER_SIZE + RANGE_RECORD_SIZE * index,
  );

  return isSpanWithin(coverage, HEADER_SIZE + RANGE_RECORD_SIZE * count, table.byteLength)
    ? rangesToIndexes(table, ranges, glyphs)
    : undefined;
}

/**
 * The coverage index of each glyph asked that a range holds: the range's start coverage index
 * plus the glyph's distance from the range's start glyph.
 *
 * @kind format
 * @param table - the layout table holding the range records, each within it
 * @param ranges - where each range record starts in the table
 * @param glyphs - the glyphs to look for
 * @returns the coverage index of each glyph asked that a range holds, by glyph
 * @see REF-OT-LAYOUT-COMMON
 */
function rangesToIndexes(
  table: DataView,
  ranges: readonly number[],
  glyphs: ReadonlySet<number>,
): CoverageIndexes {
  return Object.fromEntries(
    [...glyphs].flatMap((glyph) => {
      const range = parseGlyphRange(table, ranges, glyph);

      return range === undefined
        ? []
        : [[glyph, table.getUint16(range + START_INDEX_FIELD) + glyph - table.getUint16(range)]];
    }),
  );
}

/**
 * Reads a Coverage table for some glyphs: the coverage index of each one it covers, from a list
 * (format 1) or from ranges (format 2).
 *
 * @kind format
 * @param table - the layout table holding the Coverage table
 * @param coverage - where the Coverage table starts in it
 * @param glyphs - the glyphs to look for
 * @returns the coverage index of each glyph asked that is covered, by glyph; undefined when the
 * Coverage table passes the table's end or has another format
 * @see REF-OT-LAYOUT-COMMON
 */
export function parseCoverage(
  table: DataView,
  coverage: number,
  glyphs: ReadonlySet<number>,
): CoverageIndexes | undefined {
  const format = isSpanWithin(coverage, HEADER_SIZE, table.byteLength)
    ? table.getUint16(coverage)
    : 0;

  if (format === GLYPH_LIST) {
    return parseGlyphList(table, coverage, glyphs);
  }

  return format === GLYPH_RANGES ? parseGlyphRanges(table, coverage, glyphs) : undefined;
}
