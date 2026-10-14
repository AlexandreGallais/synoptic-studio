// The `GPOS` table built byte by byte for the tests of `src/io/kerning/` and
// `src/io/font-file/` (EN-031), with its FeatureList, LookupList, Coverage, ClassDef and PairPos subtables, each field at its offset in the OpenType
// specification. Tests only: never part of the library.

import { int16, tagBytes, uint16, uint32 } from "./font-fixtures";

/** Lookup type 2, pair adjustment; format 2 of a PairPos subtable, by classes. */
const PAIR_ADJUSTMENT = 2;

/** Lookup type 9, extension. */
const EXTENSION = 9;

/** Sizes and versions of the tables, in bytes. */
const LAYOUT = {
  extensionSize: 8,
  featureRecordSize: 6,
  gposHeaderSize: 10,
  lookupHeaderSize: 6,
  pairPos1HeaderSize: 10,
  pairPos2HeaderSize: 16,
};

/** ValueFormat X_ADVANCE: a ValueRecord of one field, `xAdvance`. */
export const X_ADVANCE_ONLY = 0x00_04;

/**
 * Concatenates parts, giving the offset of each from the start.
 *
 * @param parts - the bytes of each part
 * @returns each part's offset
 */
function offsetsOf(parts: readonly (readonly number[])[]): number[] {
  return parts.map((unused, index) =>
    parts.slice(0, index).reduce((sum, part) => sum + part.length, 0),
  );
}

/**
 * A Coverage table of format 1, a list of glyphs (`REF-OT-LAYOUT-COMMON`).
 *
 * @param glyphs - the glyphs, ascending
 * @returns its bytes
 */
export function coverageList(glyphs: readonly number[]): number[] {
  return [...uint16(1), ...uint16(glyphs.length), ...glyphs.flatMap((glyph) => uint16(glyph))];
}

/** One range of glyphs: a Coverage range or a ClassDef range. */
export type GlyphRange = {
  /** First glyph. */
  readonly start: number;
  /** Last glyph. */
  readonly end: number;
  /** Coverage index of the first glyph, or the class of the range. */
  readonly value: number;
};

/**
 * A Coverage table of format 2 or a ClassDef table of format 2: the format, the number of ranges,
 * then start, end and value for each (`REF-OT-LAYOUT-COMMON`).
 *
 * @param ranges - the ranges, ascending
 * @returns its bytes
 */
export function rangeTable(ranges: readonly GlyphRange[]): number[] {
  return [
    ...uint16(PAIR_ADJUSTMENT),
    ...uint16(ranges.length),
    ...ranges.flatMap((range) => [
      ...uint16(range.start),
      ...uint16(range.end),
      ...uint16(range.value),
    ]),
  ];
}

/** A first glyph of PairPos format 1 and its pairs, each with its ValueRecords' fields. */
export type PairSetFields = {
  /** The first glyph. */
  readonly first: number;
  /** Each second glyph, then the fields of its two ValueRecords. */
  readonly pairs: readonly (readonly [second: number, ...fields: number[]])[];
};

/**
 * A PairSet table: its number of pairs, then each second glyph and its ValueRecords' fields.
 *
 * @param set - the first glyph and its pairs
 * @returns its bytes
 */
function pairSetBytes(set: PairSetFields): number[] {
  return [
    ...uint16(set.pairs.length),
    ...set.pairs.flatMap((fields) => fields.flatMap((field) => int16(field))),
  ];
}

/**
 * A PairPos subtable of format 1: its header, a PairSet per first glyph, then a Coverage list of
 * the first glyphs (`REF-OT-GPOS`).
 *
 * @param formats - `valueFormat1` and `valueFormat2`
 * @param sets - the pair sets, by ascending first glyph
 * @returns its bytes
 */
export function pairPosFormat1(
  formats: readonly [number, number],
  sets: readonly PairSetFields[],
): number[] {
  const headerSize = LAYOUT.pairPos1HeaderSize + 2 * sets.length;
  const pairSets = sets.map((set) => pairSetBytes(set));
  const starts = offsetsOf(pairSets).map((offset) => headerSize + offset);
  const coverage = headerSize + pairSets.flat().length;

  return [
    ...uint16(1),
    ...uint16(coverage),
    ...uint16(formats[0]),
    ...uint16(formats[1]),
    ...uint16(sets.length),
    ...starts.flatMap((start) => uint16(start)),
    ...pairSets.flat(),
    ...coverageList(sets.map((set) => set.first)),
  ];
}

/** The parts of a PairPos subtable of format 2 with `xAdvance` records only. */
export type ClassPairFields = {
  /** The first glyphs covered, ascending. */
  readonly covered: readonly number[];
  /** Class ranges of the first glyphs. */
  readonly firstClasses: readonly GlyphRange[];
  /** Class ranges of the second glyphs. */
  readonly secondClasses: readonly GlyphRange[];
  /** Kerning by first class, then second class, class 0 included. */
  readonly values: readonly (readonly number[])[];
};

/**
 * A PairPos subtable of format 2, `valueFormat1` X_ADVANCE and `valueFormat2` 0: its header, the
 * Class1 records, then the Coverage list and the two ClassDef tables (`REF-OT-GPOS`).
 *
 * @param fields - its coverage, classes and values
 * @returns its bytes
 */
export function pairPosFormat2(fields: ClassPairFields): number[] {
  const records = fields.values.flatMap((row) => row.flatMap((value) => int16(value)));
  const covered = coverageList(fields.covered);
  const firstClasses = rangeTable(fields.firstClasses);
  const coverage = LAYOUT.pairPos2HeaderSize + records.length;
  const first = coverage + covered.length;
  const second = first + firstClasses.length;

  return [
    ...uint16(PAIR_ADJUSTMENT),
    ...uint16(coverage),
    ...uint16(X_ADVANCE_ONLY),
    ...uint16(0),
    ...uint16(first),
    ...uint16(second),
    ...uint16(fields.values.length),
    ...uint16(fields.values[0]?.length ?? 0),
    ...records,
    ...covered,
    ...firstClasses,
    ...rangeTable(fields.secondClasses),
  ];
}

/** A lookup of a `GPOS` table. */
export type GposLookup = {
  /** Its lookup type: 2 pair adjustment, 9 extension of pair adjustment, another ignored. */
  readonly type: number;
  /** The bytes of its subtables, of type 2 even in an extension lookup. */
  readonly subtables: readonly (readonly number[])[];
};

/**
 * A Lookup table; in an extension lookup each subtable is wrapped in an extension subtable of 8
 * bytes pointing right after itself (`REF-OT-LAYOUT-COMMON`, `REF-OT-GPOS`).
 *
 * @param lookup - its type and subtables
 * @returns its bytes
 */
function lookupTable(lookup: GposLookup): number[] {
  const subtables = lookup.subtables.map((subtable) =>
    lookup.type === EXTENSION
      ? [...uint16(1), ...uint16(PAIR_ADJUSTMENT), ...uint32(LAYOUT.extensionSize), ...subtable]
      : [...subtable],
  );
  const headerSize = LAYOUT.lookupHeaderSize + 2 * subtables.length;

  return [
    ...uint16(lookup.type),
    ...uint16(0),
    ...uint16(subtables.length),
    ...offsetsOf(subtables).flatMap((offset) => uint16(headerSize + offset)),
    ...subtables.flat(),
  ];
}

/** A feature of a `GPOS` table: its tag and the indices of its lookups. */
export type GposFeature = {
  /** Four ASCII characters. */
  readonly tag: string;
  /** Indices in the LookupList. */
  readonly lookups: readonly number[];
};

/**
 * A `GPOS` table of version 1.0: its header (the ScriptList offset left 0), the FeatureList with
 * its Feature tables, then the LookupList with its Lookup tables (`REF-OT-GPOS`).
 *
 * @param features - each feature's tag and lookup indices
 * @param lookups - the lookups, in LookupList order
 * @returns its bytes
 */
export function gposTable(
  features: readonly GposFeature[],
  lookups: readonly GposLookup[],
): number[] {
  const featureTables = features.map((feature) => [
    ...uint16(0),
    ...uint16(feature.lookups.length),
    ...feature.lookups.flatMap((index) => uint16(index)),
  ]);
  const featureHeader = 2 + LAYOUT.featureRecordSize * features.length;
  const featureList = [
    ...uint16(features.length),
    ...features.flatMap((feature, index) => [
      ...tagBytes(feature.tag),
      ...uint16(featureHeader + (offsetsOf(featureTables)[index] ?? 0)),
    ]),
    ...featureTables.flat(),
  ];
  const lookupTables = lookups.map((lookup) => lookupTable(lookup));
  const lookupList = [
    ...uint16(lookups.length),
    ...offsetsOf(lookupTables).flatMap((offset) => uint16(2 + 2 * lookups.length + offset)),
    ...lookupTables.flat(),
  ];

  return [
    ...uint16(1),
    ...uint16(0),
    ...uint16(0),
    ...uint16(LAYOUT.gposHeaderSize),
    ...uint16(LAYOUT.gposHeaderSize + featureList.length),
    ...featureList,
    ...lookupList,
  ];
}
