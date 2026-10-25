import { elementAddress, indexInRange, isSpanWithin } from "../../math";

import type { CharacterGlyph } from "../../model";

/** Offset of `numGroups` in a format 12 subtable. */
const NUM_GROUPS_OFFSET = 12;

/** Offset of the groups in a format 12 subtable, after its 16-byte header. */
const GROUPS_OFFSET = 16;

/** Size of one group: start code, end code and start glyph, four bytes each. */
const GROUP_SIZE = 12;

/** Offset of a group's end code. */
const END_CODE_FIELD = 4;

/** Offset of a group's start glyph. */
const START_GLYPH_FIELD = 8;

/** One sequential map group: consecutive characters drawn by consecutive glyphs. */
type Group = {
  /** First character code. */
  readonly start: number;
  /** Last character code. */
  readonly end: number;
  /** Glyph of the first character. */
  readonly glyph: number;
};

/**
 * Reads the sequential map groups of a format 12 subtable, if they fit in the table.
 *
 * @kind format
 * @param cmap - the font's `cmap` table
 * @param subtable - where the subtable starts in the table
 * @returns its groups, in order; undefined when its header or its groups pass the table's end
 */
function parseGroups(cmap: Readonly<DataView>, subtable: number): readonly Group[] | undefined {
  const count = isSpanWithin(subtable, GROUPS_OFFSET, cmap.byteLength)
    ? cmap.getUint32(subtable + NUM_GROUPS_OFFSET)
    : Infinity;

  return isSpanWithin(subtable, elementAddress(GROUPS_OFFSET, GROUP_SIZE, count), cmap.byteLength)
    ? Array.from({ length: count }, (unused, index) => {
        const place = elementAddress(subtable + GROUPS_OFFSET, GROUP_SIZE, index);

        return {
          end: cmap.getUint32(place + END_CODE_FIELD),
          glyph: cmap.getUint32(place + START_GLYPH_FIELD),
          start: cmap.getUint32(place),
        };
      })
    : undefined;
}

/**
 * Reads the glyph of each character from a `cmap` format 12 subtable, segmented coverage: the
 * group holding the character gives its start glyph plus the character's distance from the
 * group's start.
 *
 * @kind format
 * @param cmap - the font's `cmap` table
 * @param subtable - where the subtable starts in the table
 * @param characters - the code points to map
 * @returns each character and its glyph, 0 for a character no group holds; undefined when the
 * subtable passes the table's end
 */
export function parseSegmentedCoverageGlyphs(
  cmap: Readonly<DataView>,
  subtable: number,
  characters: readonly number[],
): readonly CharacterGlyph[] | undefined {
  const groups = parseGroups(cmap, subtable);

  return groups === undefined
    ? undefined
    : characters.map((codePoint): CharacterGlyph => {
        const group = groups.find(
          (candidate) => candidate.start <= codePoint && codePoint <= candidate.end,
        );

        return [
          codePoint,
          group === undefined ? 0 : indexInRange(codePoint, group.start, group.glyph),
        ];
      });
}
