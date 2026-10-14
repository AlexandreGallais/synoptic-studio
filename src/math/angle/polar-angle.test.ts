import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { directionAt } from "./direction-at";
import { polarAngle } from "./polar-angle";

describe("polarAngle (REF-MDN-ATAN2)", () => {
  it("is π/2 for (0, 1) and π for (−1, 0)", () => {
    expect(polarAngle({ x: 0, y: 1 })).toBe(Math.PI / 2);
    expect(polarAngle({ x: -1, y: 0 })).toBe(Math.PI);
  });

  test.prop({ angle: fc.double({ max: 3.1, min: -3.1, noNaN: true }) })(
    "is the angle of the direction at that angle",
    ({ angle }) => {
      expect(polarAngle(directionAt(angle))).toBeCloseTo(angle, 12);
    },
  );
});
