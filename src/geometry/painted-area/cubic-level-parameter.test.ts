import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { cubicPoint } from "../placement";

import { cubicLevelParameter } from "./cubic-level-parameter";

import type { Cubic } from "../contour";

/** An arch from (0, 0) to (100, 0), 30 high: y = −120 t (1 − t). */
const ARCH: Cubic = {
  end: { x: 100, y: 0 },
  first: { x: 0, y: -40 },
  kind: "cubic",
  second: { x: 100, y: -40 },
  start: { x: 0, y: 0 },
};

describe("cubicLevelParameter (DERIV-painted-area step 4)", () => {
  it("finds the arch's way down to −15 at (1 + √½) / 2", () => {
    // −120 t (1 − t) = −15: t (1 − t) = ⅛, t = (1 ± √½) / 2; on [½, 1], the + root ≈ 0.85355.
    expect(cubicLevelParameter(ARCH, [0.5, 1], -15)).toBeCloseTo((1 + Math.SQRT1_2) / 2, 12);
  });

  it("finds the arch's way up to −15 at (1 − √½) / 2", () => {
    // On [0, ½], y falls from 0 to −30: the − root ≈ 0.14645.
    expect(cubicLevelParameter(ARCH, [0, 0.5], -15)).toBeCloseTo((1 - Math.SQRT1_2) / 2, 12);
  });
});

describe("cubicLevelParameter (properties)", () => {
  test.prop({
    height: fc.integer({ max: 1000, min: 1 }),
    level: fc.double({ max: 1, min: 0, noNaN: true }),
  })("reaches the level asked on either side of an arch", ({ height, level }) => {
    // An arch of height ¾ h: its y runs over [−¾ h, 0] on each half.
    const arch = { ...ARCH, first: { x: 0, y: -height }, second: { x: 100, y: -height } };
    const y = -0.75 * height * level;

    expect(cubicPoint(arch, cubicLevelParameter(arch, [0, 0.5], y)).y).toBeCloseTo(y, 6);
    expect(cubicPoint(arch, cubicLevelParameter(arch, [0.5, 1], y)).y).toBeCloseTo(y, 6);
  });
});
