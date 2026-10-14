import { modulo } from "../../math";

import type { CharacterGlyph } from "./character-glyph";

/** Offset of `segCountX2`, twice the number of segments, in a format 4 subtable. */
const SEG_COUNT_X2_OFFSET = 6;

/** Offset of the `endCode` array in a format 4 subtable, after its 14-byte header. */
const END_CODES_OFFSET = 14;

/** Glyph indices are reckoned modulo 65 536 once the delta is added. */
const GLYPH_INDEX_MODULUS = 65_536;

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
  /** Where the segment's `idRangeOffset` sits in the file. */
  readonly rangeOffsetPlace: number;
};

/**
 * Reads the segments of a format 4 subtable. Four arrays of two bytes per segment follow its
 * header: end codes, then, past a 2-byte pad, start codes, deltas and range offsets.
 *
 * @kind format
 * @param view - the font file
 * @param subtable - where the subtable starts in the file
 * @returns its segments, in order
 * @see REF-OT-CMAP
 */
function parseSegments(view: DataView, subtable: number): readonly Segment[] {
  const arraySize = view.getUint16(subtable + SEG_COUNT_X2_OFFSET);
  const ends = subtable + END_CODES_OFFSET;
  const starts = ends + arraySize + 2;
  const deltas = starts + arraySize;
  const rangeOffsets = deltas + arraySize;

  return Array.from({ length: arraySize / 2 }, (unused, index) => ({
    delta: view.getInt16(deltas + 2 * index),
    end: view.getUint16(ends + 2 * index),
    rangeOffset: view.getUint16(rangeOffsets + 2 * index),
    rangeOffsetPlace: rangeOffsets + 2 * index,
    start: view.getUint16(starts + 2 * index),
  }));
}

/**
 * The glyph a segment's glyph array gives a character: 0 stays 0, the missing glyph; another
 * glyph is added the delta.
 *
 * @kind format
 * @param view - the font file
 * @param segment - the segment holding the character, with a range offset
 * @param codePoint - the character's code
 * @returns its glyph index
 * @see REF-OT-CMAP
 */
function glyphArrayToGlyph(view: DataView, segment: Segment, codePoint: number): number {
  const glyph = view.getUint16(
    segment.rangeOffsetPlace + segment.rangeOffset + 2 * (codePoint - segment.start),
  );

  return glyph === 0 ? 0 : modulo(glyph + segment.delta, GLYPH_INDEX_MODULUS);
}

/**
 * The glyph a segment gives a character: the code plus the delta, or the glyph array's entry.
 *
 * @kind format
 * @param view - the font file
 * @param segment - the first segment ending at or after the character, if any
 * @param codePoint - the character's code
 * @returns its glyph index; 0 when no segment holds it
 * @see REF-OT-CMAP
 */
function segmentToGlyph(view: DataView, segment: Segment | undefined, codePoint: number): number {
  if (segment === undefined || segment.start > codePoint) {
    return 0;
  }

  return segment.rangeOffset === 0
    ? modulo(codePoint + segment.delta, GLYPH_INDEX_MODULUS)
    : glyphArrayToGlyph(view, segment, codePoint);
}

/**
 * Reads the glyph of each character from a `cmap` format 4 subtable, segment mapping to delta
 * values: the first segment whose end code is at or after the character holds it, if its start
 * code is not after it.
 *
 * @kind format
 * @param view - the font file
 * @param subtable - where the subtable starts in the file
 * @param characters - the code points to map
 * @returns each character and its glyph, 0 for a character the subtable does not map
 * @see REF-OT-CMAP
 */
export function parseSegmentMappingGlyphs(
  view: DataView,
  subtable: number,
  characters: readonly number[],
): readonly CharacterGlyph[] {
  const segments = parseSegments(view, subtable);

  return characters.map((codePoint): CharacterGlyph => [
    codePoint,
    segmentToGlyph(
      view,
      segments.find((segment) => segment.end >= codePoint),
      codePoint,
    ),
  ]);
}
