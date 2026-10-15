import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { faceWeight } from "./face-weight";

describe("faceWeight (REF-OT-OS2)", () => {
  it.each([
    // Inside 100 to 900: kept as written, a weight between the hundreds too.
    { weight: 300, weightClass: 300 },
    { weight: 400, weightClass: 400 },
    { weight: 450, weightClass: 450 },
    { weight: 100, weightClass: 100 },
    { weight: 900, weightClass: 900 },
    // Outside: brought to the nearest end, 50 → 100, 950 → 900, and the ends 1 and 1000.
    { weight: 100, weightClass: 50 },
    { weight: 900, weightClass: 950 },
    { weight: 100, weightClass: 1 },
    { weight: 900, weightClass: 1000 },
  ])("[F15.AC1] gives usWeightClass $weightClass the weight $weight", ({ weight, weightClass }) => {
    expect(faceWeight(weightClass)).toBe(weight);
  });

  it("gives 100 for a usWeightClass of 0, outside the valid 1 to 1000", () => {
    expect(faceWeight(0)).toBe(100);
  });

  test.prop({ weightClass: fc.integer({ max: 65_535, min: 0 }) })(
    "gives a weight from 100 to 900, the usWeightClass itself when it lies there",
    ({ weightClass }) => {
      const weight = faceWeight(weightClass);

      expect(weight).toBeGreaterThanOrEqual(100);
      expect(weight).toBeLessThanOrEqual(900);

      if (weightClass >= 100 && weightClass <= 900) {
        expect(weight).toBe(weightClass);
      }
    },
  );
});
