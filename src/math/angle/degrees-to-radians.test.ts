import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { degreesToRadians } from "./degrees-to-radians";
import { radiansToDegrees } from "./radians-to-degrees";

describe("degreesToRadians (REF-OPENSTAX-ANGLES)", () => {
  it("gives π/2 for 90° and π/3 for 60°", () => {
    // 90π/180 and 60π/180, the product first.
    expect(degreesToRadians(90)).toBe(Math.PI / 2);
    expect(degreesToRadians(60)).toBe(1.0471975511965976);
  });

  test.prop({ degrees: fc.integer({ max: 720, min: -720 }) })(
    "comes back to the degrees within a rounding error",
    ({ degrees }) => {
      expect(radiansToDegrees(degreesToRadians(degrees))).toBeCloseTo(degrees, 9);
    },
  );
});
