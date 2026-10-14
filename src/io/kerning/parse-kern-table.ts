import { isSpanWithin } from "../../math";

import type { KerningLookup } from "../../model";

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
 * Reads where each subtable of a `kern` table starts: right after the header, then each after the
 * previous one's length.
 *
 * @kind format
 * @param kern - the `kern` table, at least its 4-byte header
 * @returns the offset of each subtable in the table, −1 for one whose header passes its end; none
 * for a version other than 0
 * @see REF-OT-KERN
 */
function parseSubtableStarts(kern: DataView): readonly number[] {
  const count = kern.getUint16(0) === 0 ? kern.getUint16(2) : 0;

  return Array.from({ length: count }).reduce<SubtableWalk>(
    (walk) =>
      isSpanWithin(walk.next, SUBTABLE_HEADER_SIZE, kern.byteLength)
        ? {
            next: walk.next + kern.getUint16(walk.next + LENGTH_FIELD),
            starts: [...walk.starts, walk.next],
          }
        : { next: walk.next, starts: [...walk.starts, -1] },
    { next: HEADER_SIZE, starts: [] },
  ).starts;
}

/**
 * Reads the right glyph and the value of each kerning pair, by right glyph.
 *
 * @kind format
 * @param kern - the `kern` table
 * @param records - where the KernPair records sit in it
 * @returns the kerning by right glyph
 * @see REF-OT-KERN
 */
function parseRightGlyphs(
  kern: DataView,
  records: readonly number[],
): Readonly<Record<number, number>> {
  return Object.fromEntries(
    records.map((record) => [
      kern.getUint16(record + RIGHT_FIELD),
      kern.getInt16(record + VALUE_FIELD),
    ]),
  );
}

/**
 * Reads the kerning pairs of a format 0 subtable whose two glyphs were asked: each its left glyph,
 * its right glyph, then its value.
 *
 * @kind format
 * @param kern - the `kern` table
 * @param subtable - where the subtable starts in it, its header within the table
 * @param glyphs - the glyphs asked, left and right
 * @returns the kerning by left glyph, then right glyph; undefined when the pairs pass the table's
 * end
 * @see REF-OT-KERN
 */
function parsePairs(
  kern: DataView,
  subtable: number,
  glyphs: ReadonlySet<number>,
): Readonly<Record<number, Readonly<Record<number, number>>>> | undefined {
  const records = parsePairRecords(kern, subtable);

  return records === undefined ? undefined : recordsToPairs(kern, records, glyphs);
}

/**
 * Reads where the KernPair records of a format 0 subtable sit.
 *
 * @kind format
 * @param kern - the `kern` table
 * @param subtable - where the subtable starts in it, its header within the table
 * @returns the offset of each record in the table; undefined when they pass its end
 * @see REF-OT-KERN
 */
function parsePairRecords(kern: DataView, subtable: number): readonly number[] | undefined {
  const count = isSpanWithin(subtable, PAIRS_OFFSET, kern.byteLength)
    ? kern.getUint16(subtable + PAIR_COUNT_FIELD)
    : Infinity;

  return isSpanWithin(subtable + PAIRS_OFFSET, PAIR_SIZE * count, kern.byteLength)
    ? Array.from({ length: count }, (unused, index) => subtable + PAIRS_OFFSET + PAIR_SIZE * index)
    : undefined;
}

/**
 * The kerning of the pairs whose two glyphs were asked, by left glyph then right glyph.
 *
 * @kind format
 * @param kern - the `kern` table
 * @param records - where the KernPair records sit in it
 * @param glyphs - the glyphs asked, left and right
 * @returns the kerning by left glyph, then right glyph
 * @see REF-OT-KERN
 */
function recordsToPairs(
  kern: DataView,
  records: readonly number[],
  glyphs: ReadonlySet<number>,
): Readonly<Record<number, Readonly<Record<number, number>>>> {
  const asked = records.filter(
    (record) =>
      glyphs.has(kern.getUint16(record)) && glyphs.has(kern.getUint16(record + RIGHT_FIELD)),
  );

  return Object.fromEntries(
    [...Map.groupBy(asked, (record) => kern.getUint16(record))].map(([left, group]) => [
      left,
      parseRightGlyphs(kern, group),
    ]),
  );
}

/**
 * Reads the kerning of a `kern` table of version 0 for some glyphs: each subtable of horizontal
 * kerning values, format 0, as a lookup of its own, since the subtables' values add up.
 *
 * @kind format
 * @param kern - the `kern` table, at least its 4-byte header
 * @param glyphs - the glyphs asked, left and right
 * @returns its kerning lookups, none for another version; undefined when a subtable passes the
 * table's end
 * @see REF-OT-KERN
 */
export function parseKernTable(
  kern: DataView,
  glyphs: ReadonlySet<number>,
): readonly KerningLookup[] | undefined {
  const starts = parseSubtableStarts(kern);
  const pairs = starts
    .filter(
      (subtable) =>
        subtable < 0 || HORIZONTAL_KERNING_COVERAGES.has(kern.getUint16(subtable + COVERAGE_FIELD)),
    )
    .map((subtable) => (subtable < 0 ? undefined : parsePairs(kern, subtable, glyphs)));
  const found = pairs.filter((subtable) => subtable !== undefined);

  return found.length < pairs.length
    ? undefined
    : found.map((subtable): KerningLookup => [{ kind: "pairs", pairs: subtable }]);
}
