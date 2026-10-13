import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { crossings } from "./crossings";

describe("crossings (REF-POMAX-BEZIER, REF-WIKI-IVT)", () => {
  it("finds both places where a function turns from negative, the closest to 0 first", () => {
    // sin(2π · 2t) < 0 on ]0.25, 0.5[ and ]0.75, 1[: it stops being negative at 0.5, then at 1.
    // Sampled from 0: it is not negative at 0, negative at 1/32: turns back at 0.5.
    expect(crossings((t) => (t - 0.3) * (t - 0.7) > 0, 1)).toEqual([expect.closeTo(0.3, 9)]);

    expect(crossings((t) => Math.abs(t - 0.5) > 0.2 && t < 0.9, 1)).toEqual([
      expect.closeTo(0.3, 9),
      expect.closeTo(0.9, 9),
    ]);
  });

  it("finds none when the function never stops being negative", () => {
    expect(crossings(() => true, 1)).toEqual([]);
  });

  test.prop({ root: fc.double({ max: 0.99, min: 0.01, noNaN: true }) })(
    "finds a single crossing anywhere in the range",
    ({ root }) => {
      expect(crossings((t) => t < root, 1)).toEqual([expect.closeTo(root, 9)]);
    },
  );
});
