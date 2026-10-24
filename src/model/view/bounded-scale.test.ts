import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { boundedScale } from "./bounded-scale";

describe("boundedScale (DERIV-view-zoom step 4)", () => {
  it.each([
    { expected: 256, scale: 300 },
    { expected: 0.01, scale: 0.001 },
    { expected: 256, scale: 256 },
    { expected: 0.01, scale: 0.01 },
    { expected: 0.01, scale: -0.05 },
  ])("takes $scale at $expected, beyond or at a bound", ({ expected, scale }) => {
    expect(boundedScale(scale)).toBe(expected);
  });

  test.prop({ scale: fc.double({ max: 256, min: 0.01, noNaN: true }) })(
    "keeps a scale within the bounds",
    ({ scale }) => {
      expect(boundedScale(scale)).toBe(scale);
    },
  );
});
