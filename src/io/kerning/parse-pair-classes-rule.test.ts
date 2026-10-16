import { describe, expect, it } from "vitest";

import { pairPosFormat2, viewOf, viewOfFirst } from "../../../tests";

import { parsePairClassesRule } from "./parse-pair-classes-rule";

import type { ClassPairFields } from "../../../tests";

/** The glyphs asked: 1 to 5. */
const GLYPHS = new Set([1, 2, 3, 4, 5]);

/**
 * Glyphs 1, 2, 5 covered; 1–2 in first class 1, 5 left in class 0. Glyph 3 in second class 1,
 * 4 in class 2.
 */
const FIELDS: ClassPairFields = {
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
};

/** The subtable of `FIELDS`. */
const SUBTABLE = viewOf(pairPosFormat2(FIELDS));

describe("parsePairClassesRule (REF-OT-GPOS)", () => {
  it("reads the covered first glyphs' classes, the second glyphs' classes and the values", () => {
    expect(parsePairClassesRule(SUBTABLE, 0, GLYPHS)).toEqual({
      firstClasses: { 1: 1, 2: 1, 5: 0 },
      kind: "classes",
      secondClasses: { 3: 1, 4: 2 },
      values: [
        [0, -5, 0],
        [0, -30, -60],
      ],
    });
  });

  it("keeps only the glyphs asked", () => {
    expect(parsePairClassesRule(SUBTABLE, 0, new Set([1, 4]))).toMatchObject({
      firstClasses: { 1: 1 },
      secondClasses: { 4: 2 },
    });
  });

  it("steps over the second glyph's record when valueFormat2 holds X_ADVANCE", () => {
    // Each Class2 record holds 4 bytes: the first xAdvance, then 99 for the second glyph.
    const subtable = pairPosFormat2({ ...FIELDS, secondAdvance: 99 });

    expect(parsePairClassesRule(viewOf(subtable), 0, GLYPHS)?.values).toEqual([
      [0, -5, 0],
      [0, -30, -60],
    ]);
  });

  it("gives undefined when a part of it passes the table's end", () => {
    const subtable = pairPosFormat2(FIELDS);

    // Cut in its header; cut in its second class definition, the last part.
    expect(parsePairClassesRule(viewOfFirst(subtable, 15), 0, GLYPHS)).toBeUndefined();
    expect(parsePairClassesRule(viewOfFirst(subtable, -2), 0, GLYPHS)).toBeUndefined();
  });

  it("gives undefined when its class counts ask for more records than the table holds", () => {
    // class1Count, at 12, set to 0xFFFF.
    const subtable = pairPosFormat2(FIELDS);

    subtable.splice(12, 2, 0xff, 0xff);

    expect(parsePairClassesRule(viewOf(subtable), 0, GLYPHS)).toBeUndefined();
  });

  it("reads records of no bytes as no kerning, without building a row per pair of classes", () => {
    // valueFormat1, at 4, set to 0, valueFormat2 already 0: every Class2 record is 0 bytes, so
    // any class counts fit; with 0xFFFF × 0xFFFF counts, at 12 and 14, a value per pair would be
    // 4.3 × 10⁹ values. A covered first glyph still matches, with 0 (REF-OT-GPOS).
    const subtable = pairPosFormat2(FIELDS);

    subtable.splice(4, 2, 0, 0);
    subtable.splice(12, 4, 0xff, 0xff, 0xff, 0xff);

    const values = parsePairClassesRule(viewOf(subtable), 0, GLYPHS)?.values;

    expect(values).toHaveLength(0xff_ff);
    expect(values?.every((row) => row.length === 0)).toBe(true);
  });
});
