import { describe, expect, it } from "vitest";

import { X_ADVANCE_ONLY, gposTable, kernTable, pairPosFormat1, viewOf } from "../../../tests";

import { parseFontKerning } from "./parse-font-kerning";

/** A `GPOS` table with a kern feature: glyphs 1 2, −50. */
const KERNING_GPOS = gposTable(
  [{ lookups: [0], tag: "kern" }],
  [
    {
      subtables: [pairPosFormat1([X_ADVANCE_ONLY, 0], [{ first: 1, pairs: [[2, -50]] }])],
      type: 2,
    },
  ],
);

/** A `GPOS` table without a kern feature. */
const MARK_GPOS = gposTable([], []);

/** A `kern` table: glyphs 1 2, −999. */
const KERN = kernTable([{ coverage: 1, pairs: [{ left: 1, right: 2, value: -999 }] }]);

/**
 * The kerning of a file holding a `GPOS` table at 0 and a `kern` table after it.
 *
 * @param gpos - the `GPOS` table's bytes
 * @param isGposRead - whether the reader is told where `GPOS` sits
 * @param isKernRead - whether the reader is told where `kern` sits
 * @returns the lookups read
 */
function kerningOf(
  gpos: readonly number[],
  isGposRead: boolean,
  isKernRead: boolean,
): ReturnType<typeof parseFontKerning> {
  return parseFontKerning(
    viewOf([...gpos, ...KERN]),
    isGposRead ? 0 : undefined,
    isKernRead ? gpos.length : undefined,
  );
}

describe("parseFontKerning (REF-CSS-FONTS-4)", () => {
  it("reads the GPOS kern feature, and then not the kern table", () => {
    expect(kerningOf(KERNING_GPOS, true, true)).toEqual([
      [{ kind: "pairs", pairs: { 1: { 2: -50 } } }],
    ]);
  });

  it("reads the kern table when GPOS has no kern feature, or no GPOS", () => {
    const fromKern = [[{ kind: "pairs", pairs: { 1: { 2: -999 } } }]];

    expect(kerningOf(MARK_GPOS, true, true)).toEqual(fromKern);
    expect(kerningOf(MARK_GPOS, false, true)).toEqual(fromKern);
  });

  it("reads no kerning from a font without either table", () => {
    expect(kerningOf(MARK_GPOS, true, false)).toEqual([]);
  });
});
