import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { firstCrossing } from "./first-crossing";

describe("firstCrossing (REF-POMAX-BEZIER, REF-WIKI-IVT)", () => {
  it("finds the first of two crossings, the one closest to 0", () => {
    // (t − 0.3)(t − 0.7) changes sign at 0.3 first.
    expect(firstCrossing((t) => (t - 0.3) * (t - 0.7) > 0, 1)).toBeCloseTo(0.3, 12);
  });

  it("finds none when the function never stops being negative", () => {
    expect(firstCrossing(() => true, 1)).toBeUndefined();
  });

  test.prop({ root: fc.double({ max: 0.99, min: 0.01, noNaN: true }) })(
    "finds a single crossing anywhere in the range",
    ({ root }) => {
      expect(firstCrossing((t) => t < root, 1)).toBeCloseTo(root, 12);
    },
  );
});
