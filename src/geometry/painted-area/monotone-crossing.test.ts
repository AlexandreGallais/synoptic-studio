import { describe, expect, it } from "vitest";

import { monotoneCrossing } from "./monotone-crossing";

import type { Cubic } from "../contour";

/**
 * An arch from (0, 0) to (100, 0), 30 high: y = −120 t (1 − t), x = 300 t² − 200 t³; y falls on
 * [0, ½] and grows on [½, 1].
 */
const ARCH: Cubic = {
  end: { x: 100, y: 0 },
  first: { x: 0, y: -40 },
  kind: "cubic",
  second: { x: 100, y: -40 },
  start: { x: 0, y: 0 },
};

describe("monotoneCrossing (DERIV-painted-area step 4)", () => {
  it("counts +1 for the way down, crossing −15 at x ≈ 94.19417, right of 50", () => {
    // t = (1 + √½) / 2 ≈ 0.85355: x = 300 t² − 200 t³ ≈ 218.56602 − 124.37185 ≈ 94.19417.
    expect(monotoneCrossing(ARCH, [0.5, 1], { x: 50, y: -15 })).toBe(1);
  });

  it("counts 0 for the way up, crossing −15 at x ≈ 5.80583, left of 50", () => {
    // t = (1 − √½) / 2 ≈ 0.14645: x ≈ 6.43398 − 0.62816 ≈ 5.80583.
    expect(monotoneCrossing(ARCH, [0, 0.5], { x: 50, y: -15 })).toBe(0);
  });

  it("counts −1 for the way up, y falling, right of 0", () => {
    expect(monotoneCrossing(ARCH, [0, 0.5], { x: 0, y: -15 })).toBe(-1);
  });

  it("counts the top in both parts, which cancel: −1 and +1 at x = 50", () => {
    // y = −30 ends the first part, falling, and starts the second, growing: both include it.
    expect(monotoneCrossing(ARCH, [0, 0.5], { x: -10, y: -30 })).toBe(-1);
    expect(monotoneCrossing(ARCH, [0.5, 1], { x: -10, y: -30 })).toBe(1);
  });

  it("counts neither part at the arch's feet, y = 0", () => {
    // The first part falls from 0: its start is excluded; the second grows to 0: its end too.
    expect(monotoneCrossing(ARCH, [0, 0.5], { x: -10, y: 0 })).toBe(0);
    expect(monotoneCrossing(ARCH, [0.5, 1], { x: -10, y: 0 })).toBe(0);
  });

  it("counts 0 above the arch", () => {
    expect(monotoneCrossing(ARCH, [0.5, 1], { x: 50, y: -35 })).toBe(0);
  });
});
