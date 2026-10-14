import { describe, expect, it } from "vitest";

import { X_ADVANCE_ONLY, pairPosFormat1, viewOf, viewOfFirst } from "../../../tests";

import { parsePairGlyphsRule } from "./parse-pair-glyphs-rule";

/** The glyphs asked: 1 to 4. */
const GLYPHS = new Set([1, 2, 3, 4]);

/** Glyph 1 then 2: −50, then 3: −20, then 9 (not asked): −5; glyph 4 then 1: 15. */
const SUBTABLE = pairPosFormat1(
  [X_ADVANCE_ONLY, 0],
  [
    {
      first: 1,
      pairs: [
        [2, -50],
        [3, -20],
        [9, -5],
      ],
    },
    { first: 4, pairs: [[1, 15]] },
  ],
);

describe("parsePairGlyphsRule (REF-OT-GPOS)", () => {
  it("reads each pair of glyphs asked, its kerning the first glyph's xAdvance", () => {
    expect(parsePairGlyphsRule(viewOf(SUBTABLE), 0, GLYPHS)).toEqual({
      kind: "pairs",
      pairs: { 1: { 2: -50, 3: -20 }, 4: { 1: 15 } },
    });
  });

  it("reads no PairSet of a first glyph not asked", () => {
    expect(parsePairGlyphsRule(viewOf(SUBTABLE), 0, new Set([4, 1]))).toEqual({
      kind: "pairs",
      pairs: { 1: {}, 4: { 1: 15 } },
    });
  });

  it("steps over the placement and the second glyph's record", () => {
    // valueFormat1 X_PLACEMENT and X_ADVANCE, valueFormat2 X_ADVANCE: each pair holds the second
    // glyph, 7 (placement), −40 (advance), then 99 for the second glyph: 2 + 4 + 2 = 8 bytes.
    const subtable = pairPosFormat1(
      [0x00_05, X_ADVANCE_ONLY],
      [
        {
          first: 1,
          pairs: [
            [2, 7, -40, 99],
            [3, 7, -10, 99],
          ],
        },
      ],
    );

    expect(parsePairGlyphsRule(viewOf(subtable), 0, GLYPHS)?.pairs).toEqual({
      1: { 2: -40, 3: -10 },
    });
  });

  it("gives undefined when a part of it passes the table's end", () => {
    // Cut in its header; cut in its second PairSet (the coverage list, 6 bytes, is cut first:
    // the table is cut by 8 bytes so that the coverage passes the end too).
    expect(parsePairGlyphsRule(viewOfFirst(SUBTABLE, 9), 0, GLYPHS)).toBeUndefined();
    expect(parsePairGlyphsRule(viewOfFirst(SUBTABLE, -8), 0, GLYPHS)).toBeUndefined();
  });

  it("gives undefined when a covered glyph has no PairSet offset", () => {
    // pairSetCount, at 8, set to 1: glyph 4, at coverage index 1, has none.
    const table = [...SUBTABLE];

    table.splice(8, 2, 0, 1);

    expect(parsePairGlyphsRule(viewOf(table), 0, GLYPHS)).toBeUndefined();
  });
});
