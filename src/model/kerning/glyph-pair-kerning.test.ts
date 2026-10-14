import { describe, expect, it } from "vitest";

import { glyphPairKerning } from "./glyph-pair-kerning";

import type { KerningLookup } from "./kerning-lookup";

/** A lookup of two subtables: glyph pairs 1 2 −50, then classes covering glyph 1, −20 for all. */
const FIRST: KerningLookup = [
  { kind: "pairs", pairs: { 1: { 2: -50 } } },
  { firstClasses: { 1: 0 }, kind: "classes", secondClasses: {}, values: [[-20]] },
];

/** A second lookup: glyph pair 1 2, −5. */
const SECOND: KerningLookup = [{ kind: "pairs", pairs: { 1: { 2: -5 } } }];

describe("glyphPairKerning (REF-OT-LAYOUT-COMMON)", () => {
  it("takes the first subtable of a lookup that matches the pair", () => {
    // 1 2 matches the pairs subtable (−50): the classes subtable is not read. 1 3 is not listed:
    // the classes subtable gives −20.
    expect(glyphPairKerning([FIRST], 1, 2)).toBe(-50);
    expect(glyphPairKerning([FIRST], 1, 3)).toBe(-20);
  });

  it("adds up the lookups", () => {
    // −50 + −5.
    expect(glyphPairKerning([FIRST, SECOND], 1, 2)).toBe(-55);
  });

  it("gives 0 when nothing matches, or with no lookup", () => {
    expect(glyphPairKerning([FIRST, SECOND], 2, 1)).toBe(0);
    expect(glyphPairKerning([], 1, 2)).toBe(0);
  });
});
