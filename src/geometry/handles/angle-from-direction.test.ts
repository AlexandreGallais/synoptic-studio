import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { angleFromDirection } from "./angle-from-direction";
import { angledHandle } from "./angled-handle";

describe("angleFromDirection (REF-MDN-ATAN2)", () => {
  it("[F12.AC4] reads 330° back from a handle above a side going right, and 0 for no handle", () => {
    expect(angleFromDirection({ x: 1, y: 0 }, { x: 25.980762113533157, y: -15 })).toBeCloseTo(
      330,
      9,
    );

    expect(angleFromDirection({ x: 0, y: 1 }, { x: 15, y: 25.980762113533157 })).toBeCloseTo(
      330,
      9,
    );

    expect(Object.is(angleFromDirection({ x: 1, y: 0 }, { x: 0, y: 0 }), 0)).toBe(true);
  });

  test.prop({ angle: fc.integer({ max: 359, min: 0 }), length: fc.integer({ max: 1000, min: 1 }) })(
    "reads back the angle a handle was built with",
    ({ angle, length }) => {
      const direction = { x: 0.6, y: -0.8 };
      const read = angleFromDirection(direction, angledHandle(direction, angle, length));

      // 359.9999999 and 0 are the same angle.
      expect(Math.min(Math.abs(read - angle), 360 - Math.abs(read - angle))).toBeCloseTo(0, 6);
    },
  );
});
