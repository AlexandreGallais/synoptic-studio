// Faces built from Inter Regular's tables, renamed and restyled, for the tests that once read
// the fonts of macOS (EN-067): a collection of faces sharing tables.
// Tests only: never part of the library.

import { builtOnce } from "./built-once";
import { fileTables } from "./collection-fixtures";
import { interRegularFile, tableOf, uint16 } from "./font-fixtures";
import { ENGLISH_US, WINDOWS_PLATFORM, nameTable } from "./name-table-fixtures";

import type { FontTables } from "./collection-fixtures";

/** Where `OS/2` holds `usWeightClass` and `fsSelection` (`REF-OT-OS2`). */
const OS2_AT = { fsSelection: 62, weightClass: 4 };

/** The tables of a font, by tag, each frozen. */
type FrozenTables = Readonly<Record<string, readonly number[]>>;

/**
 * Tables frozen, each array and their record, so that no test changes them for the next.
 *
 * @template T - a table's bytes
 * @param tables - tables, by tag
 * @returns the same tables, frozen
 */
function frozenTables<T extends object>(
  tables: Readonly<Record<string, T>>,
): Readonly<Record<string, T>> {
  return Object.freeze(
    Object.fromEntries(Object.entries(tables).map(([tag, body]) => [tag, Object.freeze(body)])),
  );
}

/**
 * Inter Regular 4.1's 17 tables, by tag, in its directory's order: GDEF, GPOS, GSUB, OS/2, cmap,
 * cvt, fpgm, gasp, glyf, head, hhea, hmtx, loca, maxp, name (1 948 bytes), post, prep. Read once
 * per test file that asks for them.
 */
const interTables: () => FrozenTables = builtOnce(() =>
  frozenTables(fileTables(interRegularFile())),
);

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
type InterFace = {
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
function interFaceTables(face: InterFace): FontTables {
  const inter = interTables();
  const os2 = inter["OS/2"] ?? [];

  return {
    ...inter,
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
 * Three faces built from Inter Regular's tables, the family Shared Sans: 400, 700 and 400 italic,
 * named `SharedSans-Regular`, `SharedSans-Bold` and `SharedSans-Italic` (name tables of 88, 82
 * and 86 bytes: 6 + 2 × 12 of header and records, 22 bytes for the family, 2 bytes a letter of the
 * PostScript name), sharing Inter's 15 other tables.
 */
const SHARED_SANS_FACES: readonly InterFace[] = [
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

/**
 * The tables of each face of `SHARED_SANS_FACES`, built by `interFaceTables` at the first call in
 * a test file, frozen, then the same arrays at every call, as a collection shares them.
 *
 * @returns the tables of each face
 */
export const sharedSansTables: () => readonly FontTables[] = builtOnce(() =>
  Object.freeze(SHARED_SANS_FACES.map((face) => frozenTables(interFaceTables(face)))),
);
