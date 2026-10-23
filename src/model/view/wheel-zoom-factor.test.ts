import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { wheelZoomFactor } from "./wheel-zoom-factor";

describe("wheelZoomFactor (DERIV-view-zoom step 5)", () => {
  it.each([
    // 1 + 0.0057 · 100; 1 / (1 + 0.0057 · 100) = 1 / 1.57; 1 + 0.0057 · 10; 1.
    { delta: -100, expected: 1.57 },
    { delta: 100, expected: 0.63694 },
    { delta: -10, expected: 1.057 },
    { delta: 0, expected: 1 },
  ])("[F42.AC1] zooms by $expected for a delta of $delta", ({ delta, expected }) => {
    expect(wheelZoomFactor(delta)).toBeCloseTo(expected, 5);
  });

  test.prop({ delta: fc.double({ max: 10_000, min: -10_000, noNaN: true }) })(
    "gives the scale back by a turn back",
    ({ delta }) => {
      expect(wheelZoomFactor(delta) * wheelZoomFactor(-delta)).toBeCloseTo(1, 9);
    },
  );
});
