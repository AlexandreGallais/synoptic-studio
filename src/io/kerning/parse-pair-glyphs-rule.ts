import { isSpanWithin } from "../../math";
import { glyphPairKey } from "../../model";

import { parseCoverage } from "./parse-coverage";
import { parseXAdvance } from "./parse-x-advance";
import { valueFormatToSize } from "./value-format-to-size";

import type { PairKerningRule } from "../../model";

/** Offset of `valueFormat1` in a PairPos subtable. */
const VALUE_FORMAT_1_OFFSET = 4;

/** Offset of `valueFormat2` in a PairPos subtable. */
const VALUE_FORMAT_2_OFFSET = 6;

/** Offset of `pairSetCount` in a PairPos format 1 subtable. */
const PAIR_SET_COUNT_OFFSET = 8;

/** Offset of `pairSetOffsets` in a PairPos format 1 subtable, after its header. */
const PAIR_SET_OFFSETS = 10;

/** Size of a pair's second glyph, of a PairSet's count and of a PairSet's offset. */
const FIELD_SIZE = 2;

/** The kerning of each pair starting with one glyph: its second glyph, then its kerning. */
type PairSet = readonly (readonly [second: number, kerning: number])[];

/** What the PairSets are read with: where they sit, the subtable's ValueFormats, the glyphs asked. */
type PairSetReading = {
  /** Where each PairSet starts in the table, by coverage index. */
  readonly places: readonly number[];
  /** ValueFormat of the first glyph's records. */
  readonly first: number;
  /** ValueFormat of the second glyph's records. */
  readonly second: number;
  /** The second glyphs to keep. */
  readonly glyphs: ReadonlySet<number>;
};

/**
 * Reads a PairSet table: the pairs starting with one glyph, each its second glyph then its two
 * ValueRecords; the first glyph's `xAdvance` is the pair's kerning. Pairs whose second glyph was
 * not asked are left out.
 *
 * @kind format
 * @param gpos - the `GPOS` table
 * @param pairSet - where the PairSet table starts in it
 * @param reading - the ValueFormats and the glyphs asked
 * @returns each second glyph asked and its kerning; undefined when the PairSet passes the table's
 * end
 * @see REF-OT-GPOS
 */
function parsePairSet(
  gpos: DataView,
  pairSet: number,
  reading: PairSetReading,
): PairSet | undefined {
  const size = FIELD_SIZE + valueFormatToSize(reading.first) + valueFormatToSize(reading.second);
  const count = isSpanWithin(pairSet, FIELD_SIZE, gpos.byteLength)
    ? gpos.getUint16(pairSet)
    : Infinity;

  return isSpanWithin(pairSet, FIELD_SIZE + size * count, gpos.byteLength)
    ? Array.from({ length: count }, (unused, index) => pairSet + FIELD_SIZE + size * index)
        .filter((record) => reading.glyphs.has(gpos.getUint16(record)))
        .map((record): readonly [number, number] => [
          gpos.getUint16(record),
          parseXAdvance(gpos, record + FIELD_SIZE, reading.first),
        ])
    : undefined;
}

/**
 * Reads where the PairSets of a PairPos format 1 subtable sit, one per coverage index.
 *
 * @kind format
 * @param gpos - the `GPOS` table
 * @param subtable - where the subtable starts in it, its header within the table
 * @returns where each PairSet starts in the table; undefined when the offsets pass its end
 * @see REF-OT-GPOS
 */
function parsePairSetPlaces(gpos: DataView, subtable: number): readonly number[] | undefined {
  const count = gpos.getUint16(subtable + PAIR_SET_COUNT_OFFSET);

  return isSpanWithin(subtable, PAIR_SET_OFFSETS + FIELD_SIZE * count, gpos.byteLength)
    ? Array.from(
        { length: count },
        (unused, index) =>
          subtable + gpos.getUint16(subtable + PAIR_SET_OFFSETS + FIELD_SIZE * index),
      )
    : undefined;
}

/**
 * Reads the PairSet of each covered first glyph, at its coverage index.
 *
 * @kind format
 * @param gpos - the `GPOS` table
 * @param covered - the coverage index of each first glyph asked, by glyph
 * @param reading - where the PairSets sit, the ValueFormats and the glyphs asked
 * @returns the rule; undefined when a PairSet is missing or passes the table's end
 * @see REF-OT-GPOS
 */
function pairSetsToRule(
  gpos: DataView,
  covered: Readonly<Record<number, number>>,
  reading: PairSetReading,
): PairKerningRule | undefined {
  const sets = Object.entries(covered).map(
    ([first, index]): readonly [string, PairSet | undefined] => {
      const place = reading.places[index];

      return [first, place === undefined ? undefined : parsePairSet(gpos, place, reading)];
    },
  );
  const found = sets.filter((set): set is readonly [string, PairSet] => set[1] !== undefined);

  return found.length < sets.length ? undefined : setsToRule(found);
}

/**
 * The rule by pairs of the PairSets read, each pair keyed by its first and second glyphs.
 *
 * @kind format
 * @param sets - each first glyph and its PairSet
 * @returns the rule by pairs
 * @see REF-OT-KERN
 */
function setsToRule(sets: readonly (readonly [string, PairSet])[]): PairKerningRule {
  return {
    kind: "pairs",
    pairs: Object.fromEntries(
      sets.flatMap(([first, set]) =>
        set.map(([second, kerning]) => [glyphPairKey(Number(first), second), kerning]),
      ),
    ),
  };
}

/**
 * Reads a pair adjustment subtable of format 1 for some glyphs: the kerning of each listed pair of
 * glyphs asked, its first glyph in the Coverage table, its PairSet at the same coverage index.
 *
 * @kind format
 * @param gpos - the `GPOS` table
 * @param subtable - where the subtable starts in it
 * @param glyphs - the glyphs asked, first and second
 * @returns the rule by glyph pairs; undefined when a part of it passes the table's end
 * @see REF-OT-GPOS
 */
export function parsePairGlyphsRule(
  gpos: DataView,
  subtable: number,
  glyphs: ReadonlySet<number>,
): PairKerningRule | undefined {
  if (!isSpanWithin(subtable, PAIR_SET_OFFSETS, gpos.byteLength)) {
    return undefined;
  }

  const covered = parseCoverage(gpos, subtable + gpos.getUint16(subtable + 2), glyphs);
  const places = parsePairSetPlaces(gpos, subtable);
  const formats = {
    first: gpos.getUint16(subtable + VALUE_FORMAT_1_OFFSET),
    second: gpos.getUint16(subtable + VALUE_FORMAT_2_OFFSET),
  };

  return covered === undefined || places === undefined
    ? undefined
    : pairSetsToRule(gpos, covered, { ...formats, glyphs, places });
}
