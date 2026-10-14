import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { clamp } from "./clamp";

describe("clamp (REF-WIKI-CLAMP)", () => {
  it.each([
    { expected: 0, value: -3 },
    { expected: 4, value: 9 },
    { expected: 2, value: 2 },
  ])("puts $value in [0, 4] at $expected", ({ expected, value }) => {
    expect(clamp(value, 0, 4)).toBe(expected);
  });

  it("gives the upper bound when the bounds cross", () => {
    // An empty list of steps: [0, −1].
    expect(clamp(0, 0, -1)).toBe(-1);
  });

  test.prop({ high: fc.integer({ max: 100, min: 0 }), value: fc.integer({ max: 200, min: -200 }) })(
    "stays in the range",
    ({ high, value }) => {
      const kept = clamp(value, 0, high);

      expect(kept >= 0 && kept <= high).toBe(true);
    },
  );
});
