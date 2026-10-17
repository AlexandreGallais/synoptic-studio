// The `post`, `OS/2` and `kern` tables built byte by byte for the tests of `src/io/font-file/`
// and `src/io/kerning/` (EN-031), each field at its offset in the OpenType specification. Tests
// only: never part of the library.

import { int16, tableOf, uint16, uint32 } from "./font-fixtures";

/** Sizes and versions of the tables, in bytes. */
const LAYOUT = {
  kernPairSize: 6,
  postVersion: 0x00_03_00_00,
};

/**
 * A `post` table: underline position at 8, thickness at 10 (`REF-OT-POST`).
 *
 * @param position - top of the underline from the baseline
 * @param thickness - its thickness
 * @returns its bytes
 */
export function postTable(position: number, thickness: number): number[] {
  return tableOf({
    fields: [
      { at: 0, bytes: uint32(LAYOUT.postVersion) },
      { at: 8, bytes: int16(position) },
      { at: 10, bytes: int16(thickness) },
    ],
    size: 32,
  });
}

/** The `OS/2` fields written by `os2Table`. */
export type Os2Fields = {
  /** Version of the table: `sCapHeight` is read from version 2. */
  readonly version: number;
  /** Height of the capitals, `sCapHeight`. */
  readonly capHeight: number;
  /** Embedding permissions. */
  readonly fsType: number;
  /** Thickness of the strike-out stroke. */
  readonly strikeoutSize: number;
  /** Top of the strike-out stroke from the baseline. */
  readonly strikeoutPosition: number;
};

/**
 * An `OS/2` table 96 bytes long, as from version 2: its version at 0, `fsType` at 8,
 * `yStrikeoutSize` at 26, `yStrikeoutPosition` at 28, `sCapHeight` at 88 (`REF-OT-OS2`).
 *
 * @param fields - the values to write
 * @returns its bytes
 */
export function os2Table(fields: Os2Fields): number[] {
  return tableOf({
    fields: [
      { at: 0, bytes: uint16(fields.version) },
      { at: 8, bytes: uint16(fields.fsType) },
      { at: 26, bytes: int16(fields.strikeoutSize) },
      { at: 28, bytes: int16(fields.strikeoutPosition) },
      { at: 88, bytes: int16(fields.capHeight) },
    ],
    size: 96,
  });
}

/** One kerning pair of a `kern` format 0 subtable. */
type KernPair = {
  /** Left glyph. */
  readonly left: number;
  /** Right glyph. */
  readonly right: number;
  /** Kerning, font units. */
  readonly value: number;
};

/** One subtable of a `kern` table: its coverage field and its pairs. */
export type KernSubtable = {
  /** Coverage field: 1 for horizontal kerning values of format 0. */
  readonly coverage: number;
  /** Its kerning pairs. */
  readonly pairs: readonly KernPair[];
};

/**
 * A `kern` table of version 0: its header, then each subtable: version, length, coverage, the
 * format 0 header (`nPairs` and three search fields), then 6 bytes per pair (`REF-OT-KERN`).
 *
 * @param subtables - each subtable's coverage and pairs
 * @returns its bytes
 */
export function kernTable(subtables: readonly KernSubtable[]): number[] {
  const header = { size: 14 };

  return [
    ...uint16(0),
    ...uint16(subtables.length),
    ...subtables.flatMap((subtable) =>
      tableOf({
        fields: [
          { at: 2, bytes: uint16(header.size + LAYOUT.kernPairSize * subtable.pairs.length) },
          { at: 4, bytes: uint16(subtable.coverage) },
          { at: 6, bytes: uint16(subtable.pairs.length) },
          {
            at: header.size,
            bytes: subtable.pairs.flatMap((pair) => [
              ...uint16(pair.left),
              ...uint16(pair.right),
              ...int16(pair.value),
            ]),
          },
        ],
        size: header.size + LAYOUT.kernPairSize * subtable.pairs.length,
      }),
    ),
  ];
}
