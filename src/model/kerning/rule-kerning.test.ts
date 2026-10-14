import { describe, expect, it } from "vitest";

import { ruleKerning } from "./rule-kerning";

import type { ClassKerningRule } from "./class-kerning-rule";
import type { PairKerningRule } from "./pair-kerning-rule";

/** Glyph 1 then glyph 2: −50. */
const PAIRS: PairKerningRule = { kind: "pairs", pairs: { 1: { 2: -50 } } };

/** Glyphs 1 and 2 covered, 1 in first class 1, 2 in class 0; glyph 3 in second class 1. */
const CLASSES: ClassKerningRule = {
  firstClasses: { 1: 1, 2: 0 },
  kind: "classes",
  secondClasses: { 3: 1 },
  values: [
    [0, -10],
    [5, -30],
  ],
};

describe("ruleKerning (REF-OT-GPOS)", () => {
  it("gives a listed pair's kerning, and nothing for a pair not listed", () => {
    expect(ruleKerning(PAIRS, 1, 2)).toBe(-50);
    expect(ruleKerning(PAIRS, 1, 3)).toBeUndefined();
    expect(ruleKerning(PAIRS, 2, 1)).toBeUndefined();
  });

  it("gives the value of the classes of a pair whose first glyph is covered", () => {
    // Glyph 1 (class 1) then glyph 3 (class 1): −30; glyph 2 (class 0) then glyph 3: −10.
    expect(ruleKerning(CLASSES, 1, 3)).toBe(-30);
    expect(ruleKerning(CLASSES, 2, 3)).toBe(-10);
  });

  it("puts a second glyph the class definition leaves out in class 0", () => {
    // Glyph 1 (class 1) then glyph 7 (class 0): 5, a match even when it is 0.
    expect(ruleKerning(CLASSES, 1, 7)).toBe(5);
    expect(ruleKerning(CLASSES, 2, 7)).toBe(0);
  });

  it("gives 0 to a first class past the rule's values", () => {
    const rule: ClassKerningRule = { ...CLASSES, firstClasses: { 9: 4 } };

    expect(ruleKerning(rule, 9, 3)).toBe(0);
  });

  it("matches nothing whose first glyph is not covered", () => {
    expect(ruleKerning(CLASSES, 3, 3)).toBeUndefined();
  });
});
