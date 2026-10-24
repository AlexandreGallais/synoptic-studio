import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { straightBoxShift } from "./straight-box-shift";

describe("straightBoxShift (ADR-0035)", () => {
  it("moves a straight box at x 4.19873 by 45.80127 to start at 50", () => {
    // A 100 × 50 turned by 30° about (60, 45): its straight box at x 60 − 55.80127 = 4.19873;
    // 50 − 4.19873 = 45.80127 (`interaction.md` §2, Each shape set).
    expect(straightBoxShift(60 - 55.80127018922194, 50)).toBeCloseTo(45.80127, 5);
  });

  it("moves backward for a target before the start, and not at all at the start", () => {
    // 10 − 40 = −30; 40 − 40 = 0.
    expect(straightBoxShift(40, 10)).toBe(-30);
    expect(straightBoxShift(40, 40)).toBe(0);
  });
});

describe("straightBoxShift (properties)", () => {
  test.prop({
    start: fc.integer({ max: 100_000, min: -100_000 }),
    target: fc.integer({ max: 100_000, min: -100_000 }),
  })("brings the start to the target", ({ start, target }) => {
    expect(start + straightBoxShift(start, target)).toBe(target);
  });
});
