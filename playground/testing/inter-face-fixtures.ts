// Faces built from Inter Regular's tables, renamed and restyled, for the tests that once read
// the fonts of macOS (EN-067): a second family beside Inter. The playground's copy of the root's
// `tests/inter-face-fixtures.ts`, the part its tests use, with `tests/built-once.ts`'s builder,
// so that they run on the playground's install alone (EN-176). Tests only.

import { fileTables, tablesFile } from "./collection-fixtures";
import { interRegularFile, tableOf, uint16 } from "./font-fixtures";
import { ENGLISH_US, WINDOWS_PLATFORM, nameTable } from "./name-table-fixtures";

import type { FontTables } from "./collection-fixtures";

/**
 * A function that builds its value at its first call and gives that same value afterwards: a
 * test file that never asks for it never builds it. The value is frozen by its builder, so that
 * no test changes it for the next.
 *
 * @template T - the value's type
 * @param build - builds the value
 * @returns the value, built once
 */
function builtOnce<T>(build: () => T): () => T {
  const built: T[] = [];

  return () => {
    if (built.length === 0) {
      built.push(build());
    }

    return built[0] ?? build();
  };
}

/** Where `head` holds `unitsPerEm` (`REF-OT-HEAD`). */
const UNITS_PER_EM_AT = 18;

/** The units per em of `secondFamilyFile`, half Inter's 2048. */
const SECOND_FAMILY_UNITS_PER_EM = 1024;

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
 * Tables with `head`'s `unitsPerEm` written anew: every advance, kerning value and line metric
 * then measures `2048 / unitsPerEm` times Inter's.
 *
 * @param tables - the tables of a face built from Inter's
 * @param unitsPerEm - from 16 to 16 384
 * @returns a copy, its `head` changed
 */
function withUnitsPerEm(tables: FontTables, unitsPerEm: number): FontTables {
  return {
    ...tables,
    head: patchedTable(interTables()["head"] ?? [], UNITS_PER_EM_AT, uint16(unitsPerEm)),
  };
}

/**
 * A second family beside Inter, built from Inter Regular's tables: named anew, 400 upright, its
 * `unitsPerEm` 1024 rather than 2048, so that each text measures twice as wide and as tall as in
 * Inter. Tagged `OTTO`, it stands for a file with CFF outlines: the editor reads that tag alone to
 * embed it as `font/otf` (`parseFontFormat`, `REF-OT-OTFF`), never its outlines.
 *
 * @param family - its family, name ID 1
 * @param sfntVersion - `TRUE_TYPE_VERSION`, or `OPEN_TYPE_VERSION` for CFF outlines
 * @returns the file's bytes
 */
export function secondFamilyFile(family: string, sfntVersion: number): Uint8Array {
  const face = interFaceTables({
    family,
    fsSelection: 0x00_c0,
    postScriptName: family.replaceAll(" ", ""),
    weightClass: 400,
  });

  return tablesFile(sfntVersion, withUnitsPerEm(face, SECOND_FAMILY_UNITS_PER_EM));
}
