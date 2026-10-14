import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { halfSize } from "./half-size";

describe("halfSize (DERIV-turned-shape)", () => {
  it("is (60, 40) for 120 × 80", () => {
    expect(halfSize({ height: 80, width: 120 })).toEqual({ x: 60, y: 40 });
  });

  test.prop({ height: fc.nat(1e5), width: fc.nat(1e5) })(
    "doubles back to the size",
    ({ height, width }) => {
      const half = halfSize({ height, width });

      expect([2 * half.x, 2 * half.y]).toEqual([width, height]);
    },
  );
});
