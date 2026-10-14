import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { degreesToRadians } from "./degrees-to-radians";
import { radiansToDegrees } from "./radians-to-degrees";

describe("radiansToDegrees (REF-OPENSTAX-ANGLES)", () => {
  it("gives 90° for π/2 and about 60° for π/3", () => {
    expect(radiansToDegrees(Math.PI / 2)).toBe(90);
    // (π/3 · 180)/π rounds to 59.99999999999999.
    expect(radiansToDegrees(Math.PI / 3)).toBe(59.99999999999999);
  });

  test.prop({ radians: fc.double({ max: 7, min: -7, noNaN: true }) })(
    "comes back to the radians within a rounding error",
    ({ radians }) => {
      expect(degreesToRadians(radiansToDegrees(radians))).toBeCloseTo(radians, 12);
    },
  );
});
