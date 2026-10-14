import { describe, expect, it } from "vitest";

import { pairKerning } from "./pair-kerning";

import type { Kerning } from "./kerning";

/** A (65) and V (86) in left class 1 and right class 1, their pair −80; T (84) in left class 2. */
const KERNING: Kerning = {
  leftClasses: { 65: 1, 84: 2, 86: 1 },
  rightClasses: { 65: 1, 86: 1 },
  values: [
    [0, 0],
    [0, -80],
    [0, -40],
  ],
};

describe("pairKerning (REF-OT-GPOS)", () => {
  it("gives the value of the pair of classes", () => {
    expect(pairKerning(KERNING, 65, 86)).toBe(-80);
    expect(pairKerning(KERNING, 84, 65)).toBe(-40);
  });

  it("gives 0 for a character in class 0 on either side", () => {
    // o (111) is in no class on either side.
    expect(pairKerning(KERNING, 111, 65)).toBe(0);
    expect(pairKerning(KERNING, 65, 111)).toBe(0);
  });

  it("gives 0 for a kerning with no values", () => {
    expect(pairKerning({ leftClasses: { 65: 3 }, rightClasses: {}, values: [] }, 65, 66)).toBe(0);
  });
});
