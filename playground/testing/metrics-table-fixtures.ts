// The `post` and `OS/2` tables built byte by byte for the tests of `src/io/font-file/` (EN-031),
// each field at its offset in the OpenType specification. The playground's copy of the root's
// `tests/metrics-table-fixtures.ts`, the part its tests use, so that they run on the playground's
// install alone (EN-176). Tests only: never part of the library.

import { int16, tableOf, uint16, uint32 } from "./font-fixtures";

/** The version of the `post` table. */
const LAYOUT = {
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
