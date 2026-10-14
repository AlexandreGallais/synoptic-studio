/** Class definition format 1: a class per glyph from a start glyph. */
const CLASS_ARRAY = 1;

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

/** A glyph and its class. */
type GlyphClass = readonly [glyph: number, group: number];

/**
 * Reads a class definition of format 1: the class of each glyph from a start glyph on.
 *
 * @kind format
 * @param view - the font file
 * @param definition - where the table starts in the file
 * @returns each glyph and its class
 * @see REF-OT-LAYOUT-COMMON
 */
function parseClassArray(view: DataView, definition: number): readonly GlyphClass[] {
  const start = view.getUint16(definition + 2);

  return Array.from(
    { length: view.getUint16(definition + GLYPH_COUNT_OFFSET) },
    (unused, index): GlyphClass => [
      start + index,
      view.getUint16(definition + CLASS_VALUES_OFFSET + 2 * index),
    ],
  );
}

/**
 * Reads a class definition of format 2: ranges of consecutive glyphs, each sharing a class.
 *
 * @kind format
 * @param view - the font file
 * @param definition - where the table starts in the file
 * @returns each glyph and its class
 * @see REF-OT-LAYOUT-COMMON
 */
function parseClassRanges(view: DataView, definition: number): readonly GlyphClass[] {
  return Array.from({ length: view.getUint16(definition + 2) }, (unused, index) => {
    const range = definition + RANGES_OFFSET + RANGE_RECORD_SIZE * index;
    const start = view.getUint16(range);

    return Array.from(
      { length: view.getUint16(range + 2) - start + 1 },
      (unusedGlyph, offset): GlyphClass => [start + offset, view.getUint16(range + CLASS_FIELD)],
    );
  }).flat();
}

/**
 * Reads a Class Definition table: the class of each glyph it assigns, from a class array
 * (format 1) or from class ranges (format 2). A glyph it does not assign is in class 0.
 *
 * @kind format
 * @param view - the font file
 * @param definition - where the table starts in the file
 * @returns the class of each glyph assigned, by glyph
 * @see REF-OT-LAYOUT-COMMON
 */
export function parseClassDefinition(
  view: DataView,
  definition: number,
): Readonly<Record<number, number>> {
  return Object.fromEntries(
    view.getUint16(definition) === CLASS_ARRAY
      ? parseClassArray(view, definition)
      : parseClassRanges(view, definition),
  );
}
