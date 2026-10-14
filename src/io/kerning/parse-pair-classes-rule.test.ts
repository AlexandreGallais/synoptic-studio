import { describe, expect, it } from "vitest";

import { pairPosFormat2, viewOf } from "../../../tests";

import { parsePairClassesRule } from "./parse-pair-classes-rule";

describe("parsePairClassesRule (REF-OT-GPOS)", () => {
  it("reads the covered first glyphs' classes, the second glyphs' classes and the values", () => {
    // Glyphs 1, 2, 5 covered; 1–2 in first class 1, 5 left in class 0. Glyph 3 in second class 1,
    // 4 in class 2.
    const subtable = pairPosFormat2({
      covered: [1, 2, 5],
      firstClasses: [{ end: 2, start: 1, value: 1 }],
      secondClasses: [
        { end: 3, start: 3, value: 1 },
        { end: 4, start: 4, value: 2 },
      ],
      values: [
        [0, -5, 0],
        [0, -30, -60],
      ],
    });

    expect(parsePairClassesRule(viewOf(subtable), 0)).toEqual({
      firstClasses: { 1: 1, 2: 1, 5: 0 },
      kind: "classes",
      secondClasses: { 3: 1, 4: 2 },
      values: [
        [0, -5, 0],
        [0, -30, -60],
      ],
    });
  });
});
