import { parseCoverage } from "./parse-coverage";
import { parseXAdvance } from "./parse-x-advance";
import { valueFormatToSize } from "./value-format-to-size";

import type { PairKerningRule } from "../../model";

/** Offset of `valueFormat1` in a PairPos subtable. */
const VALUE_FORMAT_1_OFFSET = 4;

/** Offset of `valueFormat2` in a PairPos subtable. */
const VALUE_FORMAT_2_OFFSET = 6;

/** Offset of `pairSetOffsets` in a PairPos format 1 subtable. */
const PAIR_SET_OFFSETS = 10;

/** The ValueFormats of the first and second glyph of a pair. */
type ValueFormats = {
  /** ValueFormat of the first glyph's records. */
  readonly first: number;
  /** ValueFormat of the second glyph's records. */
  readonly second: number;
};

/**
 * Reads a PairSet table: the pairs starting with one glyph, each its second glyph then its two
 * ValueRecords; the first glyph's `xAdvance` is the pair's kerning.
 *
 * @kind format
 * @param view - the font file
 * @param pairSet - where the PairSet table starts in the file
 * @param formats - the subtable's ValueFormats
 * @returns the kerning by second glyph
 * @see REF-OT-GPOS
 */
function parsePairSet(
  view: DataView,
  pairSet: number,
  formats: ValueFormats,
): Readonly<Record<number, number>> {
  const recordSize = 2 + valueFormatToSize(formats.first) + valueFormatToSize(formats.second);

  return Object.fromEntries(
    Array.from({ length: view.getUint16(pairSet) }, (unused, index) => {
      const record = pairSet + 2 + recordSize * index;

      return [view.getUint16(record), parseXAdvance(view, record + 2, formats.first)];
    }),
  );
}

/**
 * Reads a pair adjustment subtable of format 1: the kerning of each listed glyph pair, its first
 * glyph in the Coverage table, its PairSet at the same coverage index.
 *
 * @kind format
 * @param view - the font file
 * @param subtable - where the subtable starts in the file
 * @returns the rule by glyph pairs
 * @see REF-OT-GPOS
 */
export function parsePairGlyphsRule(view: DataView, subtable: number): PairKerningRule {
  const formats = {
    first: view.getUint16(subtable + VALUE_FORMAT_1_OFFSET),
    second: view.getUint16(subtable + VALUE_FORMAT_2_OFFSET),
  };
  const firsts = parseCoverage(view, subtable + view.getUint16(subtable + 2));

  return {
    kind: "pairs",
    pairs: Object.fromEntries(
      firsts.map((first, index) => [
        first,
        parsePairSet(
          view,
          subtable + view.getUint16(subtable + PAIR_SET_OFFSETS + 2 * index),
          formats,
        ),
      ]),
    ),
  };
}
