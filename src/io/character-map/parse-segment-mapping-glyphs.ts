import { elementAddress, indexInRange, isSpanWithin, modulo } from "../../math";
import { GLYPH_INDEX_COUNT } from "../../model";

import type { CharacterGlyph } from "../../model";

/** Offset of `segCountX2`, twice the number of segments, in a format 4 subtable. */
const SEG_COUNT_X2_OFFSET = 6;

/** Offset of the `endCode` array in a format 4 subtable, after its 14-byte header. */
const END_CODES_OFFSET = 14;

/** Size of the pad between the end codes and the start codes. */
const PAD_SIZE = 2;

/** Number of arrays of two bytes per segment: end codes, start codes, deltas, range offsets. */
const SEGMENT_ARRAYS = 4;

/** Size of a glyph index in the glyph array. */
const GLYPH_SIZE = 2;

/** One segment of a format 4 subtable: a range of character codes and how they map to glyphs. */
type Segment = {
  /** First character code. */
  readonly start: number;
  /** Last character code. */
  readonly end: number;
  /** `idDelta`, added to the code or to the glyph read from the glyph array. */
  readonly delta: number;
  /** `idRangeOffset`: 0, or the offset from its own place to the segment's glyphs. */
  readonly rangeOffset: number;
  /** Where the segment's `idRangeOffset` sits in the table. */
  readonly rangeOffsetPlace: number;
};

/**
 * Reads the segments of a format 4 subtable, if they fit in the table. Four arrays of two bytes
 * per segment follow its header: end codes, then, past a 2-byte pad, start codes, deltas and range
 * offsets.
 *
 * @kind format
 * @param cmap - the font's `cmap` table
 * @param subtable - where the subtable starts in the table
 * @returns its segments, in order; undefined when its header or its arrays pass the table's end
 * @see REF-OT-CMAP
 */
function parseSegments(cmap: Readonly<DataView>, subtable: number): readonly Segment[] | undefined {
  const arraySize = isSpanWithin(subtable, END_CODES_OFFSET, cmap.byteLength)
    ? cmap.getUint16(subtable + SEG_COUNT_X2_OFFSET)
    : Infinity;
  const ends = subtable + END_CODES_OFFSET;
  // The four arrays of arraySize bytes, the pad after the first: the last starts past three.
  const rangeOffsets = elementAddress(ends + PAD_SIZE, arraySize, SEGMENT_ARRAYS - 1);

  return isSpanWithin(ends, elementAddress(PAD_SIZE, arraySize, SEGMENT_ARRAYS), cmap.byteLength)
    ? Array.from({ length: arraySize / 2 }, (unused, index) => ({
        delta: cmap.getInt16(elementAddress(rangeOffsets - arraySize, 2, index)),
        end: cmap.getUint16(elementAddress(ends, 2, index)),
        rangeOffset: cmap.getUint16(elementAddress(rangeOffsets, 2, index)),
        rangeOffsetPlace: elementAddress(rangeOffsets, 2, index),
        start: cmap.getUint16(elementAddress(ends + arraySize + PAD_SIZE, 2, index)),
      }))
    : undefined;
}

/**
 * The glyph a segment's glyph array gives a character: 0 stays 0, the missing glyph; another
 * glyph is added the delta. The entry sits at the range offset's own place, plus the range offset,
 * plus twice the character's distance from the segment's start.
 *
 * @kind format
 * @param cmap - the font's `cmap` table
 * @param segment - the segment holding the character, with a range offset
 * @param codePoint - the character's code
 * @returns its glyph index; undefined when the entry lies outside the table
 * @see REF-OT-CMAP
 */
function glyphArrayToGlyph(
  cmap: Readonly<DataView>,
  segment: Segment,
  codePoint: number,
): number | undefined {
  const entry = elementAddress(
    segment.rangeOffsetPlace + segment.rangeOffset,
    GLYPH_SIZE,
    indexInRange(codePoint, segment.start, 0),
  );
  const glyph = isSpanWithin(entry, GLYPH_SIZE, cmap.byteLength) ? cmap.getUint16(entry) : -1;

  if (glyph === -1) {
    return undefined;
  }

  return glyph === 0 ? 0 : modulo(glyph + segment.delta, GLYPH_INDEX_COUNT);
}

/**
 * The glyph a segment gives a character: the code plus the delta, or the glyph array's entry.
 *
 * @kind format
 * @param cmap - the font's `cmap` table
 * @param segment - the first segment ending at or after the character, if any
 * @param codePoint - the character's code
 * @returns its glyph index, 0 when no segment holds it; undefined when its entry lies outside
 * the table
 * @see REF-OT-CMAP
 */
function segmentToGlyph(
  cmap: Readonly<DataView>,
  segment: Segment | undefined,
  codePoint: number,
): number | undefined {
  if (segment === undefined || segment.start > codePoint) {
    return 0;
  }

  return segment.rangeOffset === 0
    ? modulo(codePoint + segment.delta, GLYPH_INDEX_COUNT)
    : glyphArrayToGlyph(cmap, segment, codePoint);
}

/**
 * The glyph of each character from a format 4 subtable's segments.
 *
 * @kind format
 * @param cmap - the font's `cmap` table
 * @param segments - the subtable's segments
 * @param characters - the code points to map
 * @returns each character and its glyph; undefined when a glyph array entry lies outside the
 * table
 * @see REF-OT-CMAP
 */
function segmentsToGlyphs(
  cmap: Readonly<DataView>,
  segments: readonly Segment[],
  characters: readonly number[],
): readonly CharacterGlyph[] | undefined {
  const glyphs = characters.flatMap((codePoint): CharacterGlyph[] => {
    const glyph = segmentToGlyph(
      cmap,
      segments.find((segment) => segment.end >= codePoint),
      codePoint,
    );

    return glyph === undefined ? [] : [[codePoint, glyph]];
  });

  return glyphs.length < characters.length ? undefined : glyphs;
}

/**
 * Reads the glyph of each character from a `cmap` format 4 subtable, segment mapping to delta
 * values: the first segment whose end code is at or after the character holds it, if its start
 * code is not after it.
 *
 * @kind format
 * @param cmap - the font's `cmap` table
 * @param subtable - where the subtable starts in the table
 * @param characters - the code points to map
 * @returns each character and its glyph, 0 for a character the subtable does not map; undefined
 * when the subtable, or a glyph array entry it points to, passes the table's end
 * @see REF-OT-CMAP
 */
export function parseSegmentMappingGlyphs(
  cmap: Readonly<DataView>,
  subtable: number,
  characters: readonly number[],
): readonly CharacterGlyph[] | undefined {
  const segments = parseSegments(cmap, subtable);

  return segments === undefined ? undefined : segmentsToGlyphs(cmap, segments, characters);
}
