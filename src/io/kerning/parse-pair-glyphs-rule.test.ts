import { describe, expect, it } from "vitest";

import { X_ADVANCE_ONLY, pairPosFormat1, viewOf } from "../../../tests";

import { parsePairGlyphsRule } from "./parse-pair-glyphs-rule";

describe("parsePairGlyphsRule (REF-OT-GPOS)", () => {
  it("reads each pair's kerning, the first glyph's xAdvance", () => {
    const subtable = pairPosFormat1(
      [X_ADVANCE_ONLY, 0],
      [
        {
          first: 1,
          pairs: [
            [2, -50],
            [3, -20],
          ],
        },
        { first: 4, pairs: [[1, 15]] },
      ],
    );

    expect(parsePairGlyphsRule(viewOf(subtable), 0)).toEqual({
      kind: "pairs",
      pairs: { 1: { 2: -50, 3: -20 }, 4: { 1: 15 } },
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

    expect(parsePairGlyphsRule(viewOf(subtable), 0).pairs).toEqual({ 1: { 2: -40, 3: -10 } });
  });
});
