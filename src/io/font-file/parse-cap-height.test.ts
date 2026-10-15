import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { os2Table, viewOf, viewOfFirst } from "../../../tests";

import { parseCapHeight } from "./parse-cap-height";

import type { Os2Fields } from "../../../tests";

/** Arial 5.01's `OS/2`: version 3, `sCapHeight` 1467 (`DERIV-text-box`, cross-check). */
const ARIAL_OS2 = {
  capHeight: 1467,
  fsType: 0,
  strikeoutPosition: 0,
  strikeoutSize: 0,
  version: 3,
};

/** Arial's units per em. */
const ARIAL_UNITS_PER_EM = 2048;

/** The fallback for 2048 units per em: 0.66 em, 0.66 × 2048 = 1351.68 (`REF-CSS-INLINE-3`, A.2). */
const FALLBACK = 1351.68;

/**
 * The cap height read from an `OS/2` table built from its fields.
 *
 * @param fields - the table's fields
 * @param unitsPerEm - the font's units per em
 * @returns the cap height read, font units
 */
function capHeightOf(fields: Os2Fields, unitsPerEm: number): number {
  return parseCapHeight(viewOf(os2Table(fields)), unitsPerEm);
}

describe("parseCapHeight (REF-OT-OS2)", () => {
  it("[F14.AC5] reads sCapHeight from an OS/2 table of version 2 or later", () => {
    expect(capHeightOf(ARIAL_OS2, ARIAL_UNITS_PER_EM)).toBe(1467);

    expect(capHeightOf({ ...ARIAL_OS2, version: 2 }, ARIAL_UNITS_PER_EM)).toBe(1467);
  });

  it.each([
    { case: "version 0", fields: { ...ARIAL_OS2, version: 0 } },
    { case: "version 1", fields: { ...ARIAL_OS2, version: 1 } },
    { case: "a cap height of 0", fields: { ...ARIAL_OS2, capHeight: 0 } },
    { case: "a negative cap height", fields: { ...ARIAL_OS2, capHeight: -1 } },
  ])("[F14.AC5] falls back to 0.66 em for $case", ({ fields }) => {
    // Before version 2 the bytes at 88 are not sCapHeight, even when the table holds them.
    expect(capHeightOf(fields, ARIAL_UNITS_PER_EM)).toBeCloseTo(FALLBACK, 9);
  });

  it("falls back to 0.66 em, never an error, for a table too short to hold sCapHeight", () => {
    // Version 3 announced, but 89 bytes: sCapHeight, at 88 and 89, is cut; 0.66 × 1000 = 660.
    const os2 = viewOfFirst(os2Table(ARIAL_OS2), 89);

    expect(parseCapHeight(os2, 1000)).toBeCloseTo(660, 9);
  });

  test.prop({
    capHeight: fc.integer({ max: 32_767, min: -32_768 }),
    unitsPerEm: fc.integer({ max: 16_384, min: 16 }),
    version: fc.integer({ max: 5, min: 0 }),
  })("gives sCapHeight when the table gives one above 0, otherwise 0.66 em", (fields) => {
    const isGiven = fields.version >= 2 && fields.capHeight > 0;

    expect(capHeightOf({ ...ARIAL_OS2, ...fields }, fields.unitsPerEm)).toBeCloseTo(
      isGiven ? fields.capHeight : 0.66 * fields.unitsPerEm,
      9,
    );
  });
});
