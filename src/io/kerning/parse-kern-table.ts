import type { KerningLookup } from "../../model";

/** Size of the `kern` header: version and `nTables`. */
const HEADER_SIZE = 4;

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
  /** Where each subtable met starts. */
  readonly starts: readonly number[];
};

/**
 * Reads where each subtable of a `kern` table starts: right after the header, then each after the
 * previous one's length.
 *
 * @kind format
 * @param view - the font file
 * @param kern - where the `kern` table starts in the file
 * @returns the offset of each subtable in the file; none for a version other than 0
 * @see REF-OT-KERN
 */
function parseSubtableStarts(view: DataView, kern: number): readonly number[] {
  const count = view.getUint16(kern) === 0 ? view.getUint16(kern + 2) : 0;

  return Array.from({ length: count }).reduce<SubtableWalk>(
    (walk) => ({
      next: walk.next + view.getUint16(walk.next + LENGTH_FIELD),
      starts: [...walk.starts, walk.next],
    }),
    { next: kern + HEADER_SIZE, starts: [] },
  ).starts;
}

/**
 * Reads the right glyph and the value of each kerning pair, by right glyph.
 *
 * @kind format
 * @param view - the font file
 * @param records - where the KernPair records sit in the file
 * @returns the kerning by right glyph
 * @see REF-OT-KERN
 */
function parseRightGlyphs(
  view: DataView,
  records: readonly number[],
): Readonly<Record<number, number>> {
  return Object.fromEntries(
    records.map((record) => [
      view.getUint16(record + RIGHT_FIELD),
      view.getInt16(record + VALUE_FIELD),
    ]),
  );
}

/**
 * Reads the kerning pairs of a format 0 subtable: each its left glyph, its right glyph, then its
 * value.
 *
 * @kind format
 * @param view - the font file
 * @param subtable - where the subtable starts in the file
 * @returns the kerning by left glyph, then right glyph
 * @see REF-OT-KERN
 */
function parsePairs(
  view: DataView,
  subtable: number,
): Readonly<Record<number, Readonly<Record<number, number>>>> {
  const records = Array.from(
    { length: view.getUint16(subtable + PAIR_COUNT_FIELD) },
    (unused, index) => subtable + PAIRS_OFFSET + PAIR_SIZE * index,
  );

  return Object.fromEntries(
    [...Map.groupBy(records, (record) => view.getUint16(record))].map(([left, group]) => [
      left,
      parseRightGlyphs(view, group),
    ]),
  );
}

/**
 * Reads the kerning of a `kern` table of version 0: each subtable of horizontal kerning values,
 * format 0, as a lookup of its own, since the subtables' values add up.
 *
 * @kind format
 * @param view - the font file
 * @param kern - where the `kern` table starts in the file
 * @returns its kerning lookups; none for another version
 * @see REF-OT-KERN
 */
export function parseKernTable(view: DataView, kern: number): readonly KerningLookup[] {
  return parseSubtableStarts(view, kern)
    .filter((subtable) =>
      HORIZONTAL_KERNING_COVERAGES.has(view.getUint16(subtable + COVERAGE_FIELD)),
    )
    .map((subtable): KerningLookup => [{ kind: "pairs", pairs: parsePairs(view, subtable) }]);
}
