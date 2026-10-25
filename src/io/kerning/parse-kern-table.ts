import { elementAddress, isSpanWithin } from "../../math";
import { glyphPairKey } from "../../model";

import type { KerningLookup, PairKerningRule } from "../../model";

/** Size of the `kern` header: version and `nTables`. */
const HEADER_SIZE = 4;

/** Size of a subtable's header: version, length and coverage. */
const SUBTABLE_HEADER_SIZE = 6;

/** Offset of a subtable's length field. */
const LENGTH_FIELD = 2;

/** Offset of a subtable's coverage field. */
const COVERAGE_FIELD = 4;

/** Offset of `nPairs` in a format 0 subtable. */
const PAIR_COUNT_FIELD = 6;

/** Offset of `kernPairs` in a format 0 subtable, after its 14 bytes of headers. */
const PAIRS_OFFSET = 14;

/** Size of one KernPair record: left glyph, right glyph, value. */
const PAIR_SIZE = 6;

/** Offset of a KernPair's right glyph. */
const RIGHT_FIELD = 2;

/** Offset of a KernPair's value. */
const VALUE_FIELD = 4;

/** Coverage bit 0, horizontal data. */
const HORIZONTAL = 0b0001;

/** Coverage bit 3, override: the subtable's values replace those accumulated. */
const OVERRIDE = 0b1000;

/**
 * Coverage fields of a subtable of horizontal kerning values in format 0: bit 0 (horizontal) set;
 * bits 1 (minimum values) and 2 (cross-stream) clear; bit 3 (override) either; the reserved bits
 * 4 to 7 and the format, bits 8 to 15, zero.
 */
const HORIZONTAL_KERNING_COVERAGES: ReadonlySet<number> = new Set([
  HORIZONTAL,
  HORIZONTAL + OVERRIDE,
]);

/** The walk through the subtables: where each one met starts, and where the next one does. */
type SubtableWalk = {
  /** Where the next subtable starts. */
  readonly next: number;
  /** Where each subtable met starts; −1 for one whose header passes the table's end. */
  readonly starts: readonly number[];
};

/**
 * Reads the length of a subtable of a `kern` table, its header included.
 *
 * @kind format
 * @param kern - the `kern` table
 * @param subtable - where the subtable starts in it
 * @returns its length in bytes; 0 when its header passes the table's end
 */
function parseSubtableLength(kern: Readonly<DataView>, subtable: number): number {
  return isSpanWithin(subtable, SUBTABLE_HEADER_SIZE, kern.byteLength)
    ? kern.getUint16(subtable + LENGTH_FIELD)
    : 0;
}

/**
 * Reads where each subtable of a `kern` table starts: right after the header, then each after the
 * previous one's length.
 *
 * @kind format
 * @param kern - the `kern` table, at least its 4-byte header
 * @returns the offset of each subtable in the table, −1 for one whose header passes its end or
 * whose length is shorter than its header, or alone when the table cannot hold the headers of the
 * subtables announced; none for a version other than 0
 */
function parseSubtableStarts(kern: Readonly<DataView>): readonly number[] {
  const count = kern.getUint16(0) === 0 ? kern.getUint16(2) : 0;

  if (!isSpanWithin(0, elementAddress(HEADER_SIZE, SUBTABLE_HEADER_SIZE, count), kern.byteLength)) {
    return [-1];
  }

  return Array.from({ length: count }).reduce<SubtableWalk>(
    (walk) => {
      const length = parseSubtableLength(kern, walk.next);

      return length >= SUBTABLE_HEADER_SIZE
        ? { next: walk.next + length, starts: [...walk.starts, walk.next] }
        : { next: walk.next, starts: [...walk.starts, -1] };
    },
    { next: HEADER_SIZE, starts: [] },
  ).starts;
}

/** One kerning pair of a subtable, and whether its value replaces the value accumulated. */
type KernStep = {
  /** The left glyph. */
  readonly left: number;
  /** The right glyph. */
  readonly right: number;
  /** The kerning, font units. */
  readonly value: number;
  /** Whether the subtable's override bit is set. */
  readonly isOverride: boolean;
};

/**
 * Reads where the KernPair records of a format 0 subtable sit.
 *
 * @kind format
 * @param kern - the `kern` table
 * @param subtable - where the subtable starts in it, its header within the table
 * @returns the offset of each record in the table; undefined when they pass its end
 */
function parsePairRecords(
  kern: Readonly<DataView>,
  subtable: number,
): readonly number[] | undefined {
  const count = isSpanWithin(subtable, PAIRS_OFFSET, kern.byteLength)
    ? kern.getUint16(subtable + PAIR_COUNT_FIELD)
    : Infinity;

  return isSpanWithin(subtable, elementAddress(PAIRS_OFFSET, PAIR_SIZE, count), kern.byteLength)
    ? Array.from({ length: count }, (unused, index) =>
        elementAddress(subtable + PAIRS_OFFSET, PAIR_SIZE, index),
      )
    : undefined;
}

/**
 * Reads the kerning pairs of a format 0 subtable whose two glyphs were asked: each its left glyph,
 * its right glyph, then its value, with the subtable's override bit.
 *
 * @kind format
 * @param kern - the `kern` table
 * @param subtable - where the subtable starts in it, −1 for one whose header passes its end
 * @param glyphs - the glyphs asked, left and right
 * @returns its pairs, in order; undefined when they pass the table's end
 */
function parseSteps(
  kern: Readonly<DataView>,
  subtable: number,
  glyphs: ReadonlySet<number>,
): readonly KernStep[] | undefined {
  if (subtable < 0) {
    return undefined;
  }

  const records = parsePairRecords(kern, subtable);
  const isOverride = kern.getUint16(subtable + COVERAGE_FIELD) === HORIZONTAL + OVERRIDE;

  return records
    ?.map((record) => ({
      isOverride,
      left: kern.getUint16(record),
      right: kern.getUint16(record + RIGHT_FIELD),
      value: kern.getInt16(record + VALUE_FIELD),
    }))
    .filter((step) => glyphs.has(step.left) && glyphs.has(step.right));
}

/**
 * The kerning of one pair from its values in the subtables, in order: each added to the value
 * accumulated, or replacing it when its subtable's override bit is set.
 *
 * @kind format
 * @param steps - the pair's values, subtable after subtable
 * @returns the kerning accumulated, font units
 */
function stepsToKerning(steps: readonly KernStep[]): number {
  return steps.reduce((sum, step) => (step.isOverride ? step.value : sum + step.value), 0);
}

/**
 * The kerning of a `kern` table's pairs as one rule by pairs, each pair's values accumulated in
 * subtable order.
 *
 * @kind format
 * @param steps - every pair of every subtable, in subtable order
 * @returns the rule, by pair key
 */
function stepsToRule(steps: readonly KernStep[]): PairKerningRule {
  return {
    kind: "pairs",
    pairs: Object.fromEntries(
      [...Map.groupBy(steps, (step) => glyphPairKey(step.left, step.right))].map(
        ([key, values]) => [key, stepsToKerning(values)],
      ),
    ),
  };
}

/**
 * Reads the kerning of a `kern` table of version 0 for some glyphs: its subtables of horizontal
 * kerning values, format 0, accumulated in order into one lookup; a subtable's values add to those
 * before it, or replace them when its override bit is set.
 *
 * @kind format
 * @param kern - the `kern` table, at least its 4-byte header
 * @param glyphs - the glyphs asked, left and right
 * @returns its kerning lookup, none for another version or without such subtables; undefined when
 * a subtable passes the table's end or is shorter than its header
 */
export function parseKernTable(
  kern: Readonly<DataView>,
  glyphs: ReadonlySet<number>,
): readonly KerningLookup[] | undefined {
  const subtables = parseSubtableStarts(kern)
    .filter(
      (subtable) =>
        subtable < 0 || HORIZONTAL_KERNING_COVERAGES.has(kern.getUint16(subtable + COVERAGE_FIELD)),
    )
    .map((subtable) => parseSteps(kern, subtable, glyphs));
  const found = subtables.filter((steps): steps is readonly KernStep[] => steps !== undefined);

  if (found.length < subtables.length) {
    return undefined;
  }

  return found.length === 0 ? [] : [[stepsToRule(found.flat())]];
}
