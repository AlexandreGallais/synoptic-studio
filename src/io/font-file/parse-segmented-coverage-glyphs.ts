import type { CharacterGlyph } from "./character-glyph";

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
 * Reads the sequential map groups of a format 12 subtable.
 *
 * @kind format
 * @param view - the font file
 * @param subtable - where the subtable starts in the file
 * @returns its groups, in order
 * @see REF-OT-CMAP
 */
function parseGroups(view: DataView, subtable: number): readonly Group[] {
  return Array.from({ length: view.getUint32(subtable + NUM_GROUPS_OFFSET) }, (unused, index) => {
    const place = subtable + GROUPS_OFFSET + GROUP_SIZE * index;

    return {
      end: view.getUint32(place + END_CODE_FIELD),
      glyph: view.getUint32(place + START_GLYPH_FIELD),
      start: view.getUint32(place),
    };
  });
}

/**
 * Reads the glyph of each character from a `cmap` format 12 subtable, segmented coverage: the
 * group holding the character gives its start glyph plus the character's distance from the
 * group's start.
 *
 * @kind format
 * @param view - the font file
 * @param subtable - where the subtable starts in the file
 * @param characters - the code points to map
 * @returns each character and its glyph, 0 for a character no group holds
 * @see REF-OT-CMAP
 */
export function parseSegmentedCoverageGlyphs(
  view: DataView,
  subtable: number,
  characters: readonly number[],
): readonly CharacterGlyph[] {
  const groups = parseGroups(view, subtable);

  return characters.map((codePoint): CharacterGlyph => {
    const group = groups.find(
      (candidate) => candidate.start <= codePoint && codePoint <= candidate.end,
    );

    return [codePoint, group === undefined ? 0 : group.glyph + codePoint - group.start];
  });
}
