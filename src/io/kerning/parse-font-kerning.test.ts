import { describe, expect, it } from "vitest";

import { X_ADVANCE_ONLY, gposTable, kernTable, pairPosFormat1, viewOf } from "../../../tests";
import { glyphPairKey } from "../../model";

import { parseFontKerning } from "./parse-font-kerning";

/** The glyphs asked. */
const GLYPHS = new Set([1, 2]);

/** A `GPOS` table with a kern feature: glyphs 1 2, −50. */
const KERNING_GPOS = viewOf(
  gposTable(
    [{ lookups: [0], tag: "kern" }],
    [
      {
        subtables: [pairPosFormat1([X_ADVANCE_ONLY, 0], [{ first: 1, pairs: [[2, -50]] }])],
        type: 2,
      },
    ],
  ),
);

/** A `GPOS` table without a kern feature. */
const MARK_GPOS = viewOf(gposTable([], []));

/** A `kern` table: glyphs 1 2, −999. */
const KERN = viewOf(kernTable([{ coverage: 1, pairs: [{ left: 1, right: 2, value: -999 }] }]));

/** The lookups of `KERN`. */
const FROM_KERN = [[{ kind: "pairs", pairs: { [glyphPairKey(1, 2)]: -999 } }]];

describe("parseFontKerning (REF-CSS-FONTS-4)", () => {
  it("reads the GPOS kern feature, and then not the kern table", () => {
    expect(parseFontKerning(KERNING_GPOS, KERN, GLYPHS)).toEqual([
      [{ kind: "pairs", pairs: { [glyphPairKey(1, 2)]: -50 } }],
    ]);
  });

  it("reads the kern table when GPOS has no kern feature, or no GPOS", () => {
    expect(parseFontKerning(MARK_GPOS, KERN, GLYPHS)).toEqual(FROM_KERN);
    expect(parseFontKerning(undefined, KERN, GLYPHS)).toEqual(FROM_KERN);
  });

  it("reads no kerning from a font without either table", () => {
    expect(parseFontKerning(MARK_GPOS, undefined, GLYPHS)).toEqual([]);
  });

  it("gives undefined for a GPOS table passing its end, without reading the kern table", () => {
    const cut = new DataView(KERNING_GPOS.buffer, 0, KERNING_GPOS.byteLength - 1);

    expect(parseFontKerning(cut, KERN, GLYPHS)).toBeUndefined();
  });
});
