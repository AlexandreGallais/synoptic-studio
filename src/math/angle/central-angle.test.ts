import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { centralAngle } from "./central-angle";

describe("centralAngle (DERIV-stretched-polygon)", () => {
  it("is π/2 per side of a square, 2π after its 4 sides", () => {
    expect(centralAngle(1, 4)).toBe(Math.PI / 2);
    expect(centralAngle(4, 4)).toBe(2 * Math.PI);
  });

  test.prop({ corners: fc.integer({ max: 12, min: 3 }) })(
    "is half of one side's angle for π/n, exactly",
    ({ corners }) => {
      expect(centralAngle(1, corners) / 2).toBe(Math.PI / corners);
    },
  );
});
