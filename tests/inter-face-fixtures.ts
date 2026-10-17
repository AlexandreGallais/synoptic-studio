// Faces built from Inter Regular's tables, renamed and restyled, for the tests that once read
// the fonts of macOS (EN-067): a collection of faces sharing tables, a second family beside Inter.
// Tests only: never part of the library.

import { fileTables } from "./collection-fixtures";
import { INTER_REGULAR_FILE, tableOf, uint16 } from "./font-fixtures";
import { ENGLISH_US, WINDOWS_PLATFORM, nameTable } from "./name-table-fixtures";

import type { FontTables } from "./collection-fixtures";

/** Where `head` holds `unitsPerEm` (`REF-OT-HEAD`). */
const UNITS_PER_EM_AT = 18;

/** Where `OS/2` holds `usWeightClass` and `fsSelection` (`REF-OT-OS2`). */
const OS2_AT = { fsSelection: 62, weightClass: 4 };

/**
 * Inter Regular 4.1's 17 tables, by tag, in its directory's order: GDEF, GPOS, GSUB, OS/2, cmap,
 * cvt, fpgm, gasp, glyf, head, hhea, hmtx, loca, maxp, name (1 948 bytes), post, prep.
 */
export const INTER_TABLES: Readonly<Record<string, readonly number[]>> =
  fileTables(INTER_REGULAR_FILE);

/**
 * A table with some of its bytes written anew.
 *
 * @param table - its bytes
 * @param at - where the bytes go
 * @param bytes - the bytes written
 * @returns a copy of the table
 */
function patchedTable(table: readonly number[], at: number, bytes: readonly number[]): number[] {
  return tableOf({
    fields: [
      { at: 0, bytes: table },
      { at, bytes },
    ],
    size: table.length,
  });
}

/** A face built from Inter Regular's tables, named and styled anew. */
export type InterFace = {
  /** Its family, name ID 1. */
  readonly family: string;
  /** Its PostScript name, name ID 6. */
  readonly postScriptName: string;
  /** Its `usWeightClass`. */
  readonly weightClass: number;
  /** Its `fsSelection`: Inter's 0x00c0 (bits 6 regular and 7), 0x00a0 bold, 0x0081 italic. */
  readonly fsSelection: number;
};

/**
 * The tables of a face built from Inter Regular's: its own `name` (name IDs 1 and 6) and `OS/2`
 * (`usWeightClass` at 4, `fsSelection` at 62), every other table Inter's very array, so that
 * faces built so share them.
 *
 * @param face - its names, weight and style
 * @returns its tables, by tag, in Inter's order
 */
export function interFaceTables(face: InterFace): FontTables {
  const os2 = INTER_TABLES["OS/2"] ?? [];

  return {
    ...INTER_TABLES,
    name: nameTable([
      {
        encoding: 1,
        language: ENGLISH_US,
        nameId: 1,
        platform: WINDOWS_PLATFORM,
        text: face.family,
      },
      {
        encoding: 1,
        language: ENGLISH_US,
        nameId: 6,
        platform: WINDOWS_PLATFORM,
        text: face.postScriptName,
      },
    ]),
    "OS/2": patchedTable(
      patchedTable(os2, OS2_AT.weightClass, uint16(face.weightClass)),
      OS2_AT.fsSelection,
      uint16(face.fsSelection),
    ),
  };
}

/**
 * Tables with `head`'s `unitsPerEm` written anew: every advance, kerning value and line metric
 * then measures `2048 / unitsPerEm` times Inter's.
 *
 * @param tables - the tables of a face built from Inter's
 * @param unitsPerEm - from 16 to 16 384
 * @returns a copy, its `head` changed
 */
export function withUnitsPerEm(tables: FontTables, unitsPerEm: number): FontTables {
  return {
    ...tables,
    head: patchedTable(INTER_TABLES["head"] ?? [], UNITS_PER_EM_AT, uint16(unitsPerEm)),
  };
}

/**
 * Three faces built from Inter Regular's tables, the family Shared Sans: 400, 700 and 400 italic,
 * named `SharedSans-Regular`, `SharedSans-Bold` and `SharedSans-Italic` (name tables of 88, 82
 * and 86 bytes: 6 + 2 × 12 of header and records, 22 bytes for the family, 2 bytes a letter of the
 * PostScript name), sharing Inter's 15 other tables.
 */
export const SHARED_SANS_FACES: readonly InterFace[] = [
  {
    family: "Shared Sans",
    fsSelection: 0x00_c0,
    postScriptName: "SharedSans-Regular",
    weightClass: 400,
  },
  {
    family: "Shared Sans",
    fsSelection: 0x00_a0,
    postScriptName: "SharedSans-Bold",
    weightClass: 700,
  },
  {
    family: "Shared Sans",
    fsSelection: 0x00_81,
    postScriptName: "SharedSans-Italic",
    weightClass: 400,
  },
];

/** The tables of each face of `SHARED_SANS_FACES`, built by `interFaceTables`. */
export const SHARED_SANS_TABLES: readonly FontTables[] = SHARED_SANS_FACES.map((face) =>
  interFaceTables(face),
);
