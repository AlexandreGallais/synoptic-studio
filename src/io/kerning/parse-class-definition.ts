import { isSpanWithin } from "../../math";

import { parseGlyphRange } from "./parse-glyph-range";

/** Class definition format 1: a class per glyph from a start glyph. */
const CLASS_ARRAY = 1;

/** Class definition format 2: ranges of glyphs sharing a class. */
const CLASS_RANGES = 2;

/** Offset of `glyphCount` in format 1, after the format and the start glyph. */
const GLYPH_COUNT_OFFSET = 4;

/** Offset of `classValues` in format 1, after the format, the start glyph and the count. */
const CLASS_VALUES_OFFSET = 6;

/** Offset of the first range record in format 2, after the format and the count. */
const RANGES_OFFSET = 4;

/** Size of one class range record: start glyph, end glyph, class. */
const RANGE_RECORD_SIZE = 6;

/** Offset of a range's class in its record, after its start and end glyphs. */
const CLASS_FIELD = 4;

/** Size of one class value. */
const CLASS_SIZE = 2;

/** The class of each glyph assigned one, by glyph. */
type GlyphClasses = Readonly<Record<number, number>>;

/**
 * Reads a class array (format 1) for some glyphs: the class of each one from the start glyph on.
 *
 * @kind format
 * @param table - the layout table holding the ClassDef table
 * @param definition - where the ClassDef table starts in it
 * @param glyphs - the glyphs to look for
 * @returns the class of each glyph asked that the array assigns; undefined when the array passes
 * the table's end
 * @see REF-OT-LAYOUT-COMMON
 */
function parseClassArray(
  table: DataView,
  definition: number,
  glyphs: ReadonlySet<number>,
): GlyphClasses | undefined {
  const start = table.getUint16(definition + 2);
  const count = isSpanWithin(definition, CLASS_VALUES_OFFSET, table.byteLength)
    ? table.getUint16(definition + GLYPH_COUNT_OFFSET)
    : Infinity;

  return isSpanWithin(definition, CLASS_VALUES_OFFSET + CLASS_SIZE * count, table.byteLength)
    ? Object.fromEntries(
        [...glyphs]
          .filter((glyph) => isSpanWithin(glyph - start, 1, count))
          .map((glyph) => [
            glyph,
            table.getUint16(definition + CLASS_VALUES_OFFSET + CLASS_SIZE * (glyph - start)),
          ]),
      )
    : undefined;
}

/**
 * Reads class ranges (format 2) for some glyphs: each glyph asked is looked up in the ranges; no
 * range is spread out glyph by glyph.
 *
 * @kind format
 * @param table - the layout table holding the ClassDef table
 * @param definition - where the ClassDef table starts in it
 * @param glyphs - the glyphs to look for
 * @returns the class of each glyph asked that a range assigns; undefined when the ranges pass
 * the table's end
 * @see REF-OT-LAYOUT-COMMON
 */
function parseClassRanges(
  table: DataView,
  definition: number,
  glyphs: ReadonlySet<number>,
): GlyphClasses | undefined {
  const count = table.getUint16(definition + 2);
  const ranges = Array.from(
    { length: count },
    (unused, index) => definition + RANGES_OFFSET + RANGE_RECORD_SIZE * index,
  );

  return isSpanWithin(definition, RANGES_OFFSET + RANGE_RECORD_SIZE * count, table.byteLength)
    ? Object.fromEntries(
        [...glyphs].flatMap((glyph) => {
          const range = parseGlyphRange(table, ranges, glyph);

          return range === undefined ? [] : [[glyph, table.getUint16(range + CLASS_FIELD)]];
        }),
      )
    : undefined;
}

/**
 * Reads a Class Definition table for some glyphs: the class of each one it assigns, from a class
 * array (format 1) or from class ranges (format 2). A glyph it does not assign is in class 0.
 *
 * @kind format
 * @param table - the layout table holding the ClassDef table
 * @param definition - where the ClassDef table starts in it
 * @param glyphs - the glyphs to look for
 * @returns the class of each glyph asked that it assigns, by glyph; undefined when it passes the
 * table's end or has another format
 * @see REF-OT-LAYOUT-COMMON
 */
export function parseClassDefinition(
  table: DataView,
  definition: number,
  glyphs: ReadonlySet<number>,
): GlyphClasses | undefined {
  const format = isSpanWithin(definition, RANGES_OFFSET, table.byteLength)
    ? table.getUint16(definition)
    : 0;

  if (format === CLASS_ARRAY) {
    return parseClassArray(table, definition, glyphs);
  }

  return format === CLASS_RANGES ? parseClassRanges(table, definition, glyphs) : undefined;
}
