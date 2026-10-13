import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { withoutNegativeZero } from "./without-negative-zero";

describe("withoutNegativeZero", () => {
  it("turns −0 into +0", () => {
    // −0 + +0 is +0 (REF-ECMA-262-NUMBER-ADD).
    const vector = withoutNegativeZero({ x: -0, y: -0 });

    expect(Object.is(vector.x, 0) && Object.is(vector.y, 0)).toBe(true);
  });

  test.prop({ x: fc.double({ noNaN: true }), y: fc.double({ noNaN: true }) })(
    "leaves every other number as it is",
    ({ x, y }) => {
      const vector = withoutNegativeZero({ x, y });

      expect(vector.x === x && vector.y === y).toBe(true);
    },
  );
});
