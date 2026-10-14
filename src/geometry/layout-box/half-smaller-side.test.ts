import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { halfSmallerSide } from "./half-smaller-side";

describe("halfSmallerSide (DERIV-round-shape)", () => {
  it("is 40 for 120 × 80 and 40.5 for 81 × 120", () => {
    expect(halfSmallerSide({ height: 80, width: 120 })).toBe(40);
    expect(halfSmallerSide({ height: 120, width: 81 })).toBe(40.5);
  });

  test.prop({ height: fc.nat(1e5), width: fc.nat(1e5) })(
    "is the same for the box turned a quarter",
    ({ height, width }) => {
      expect(halfSmallerSide({ height, width })).toBe(
        halfSmallerSide({ height: width, width: height }),
      );
    },
  );
});
