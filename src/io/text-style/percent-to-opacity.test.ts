import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { percentToOpacity } from "./percent-to-opacity";

describe("percentToOpacity (REF-SVG2-FILL-OPACITY)", () => {
  it("[F05.AC2] writes 0 %, 50 % and 100 % as 0, 0.5 and 1", () => {
    expect([0, 50, 100].map((percent) => percentToOpacity(percent))).toEqual([0, 0.5, 1]);
  });

  test.prop({ percent: fc.integer({ max: 100, min: 0 }) })(
    "stays from 0 to 1, a hundredth per percent",
    ({ percent }) => {
      const opacity = percentToOpacity(percent);

      expect(opacity).toBeGreaterThanOrEqual(0);
      expect(opacity).toBeLessThanOrEqual(1);
      expect(opacity * 100).toBeCloseTo(percent, 9);
    },
  );
});
