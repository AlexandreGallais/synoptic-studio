import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { arcChordSide } from "./arc-chord-side";

/** The fillet of `isInCircularSegment`'s tests: from (0, 20) up to (20, 0) about (20, 20). */
const FILLET = { end: { x: 20, y: 0 }, positive: true, start: { x: 0, y: 20 } };

/** The same fillet travelled the other way. */
const REVERSED = { end: FILLET.start, positive: false, start: FILLET.end };

/** A coordinate, an integer, so that the perp dot product is exact. */
const COORDINATE = fc.integer({ max: 40, min: -20 });

describe("arcChordSide (DERIV-painted-area step 3)", () => {
  it("is negative beyond the chord, positive on the centre's side, 0 on it", () => {
    // E − S = (20, −20). (8, 8) − S = (8, −12): 20 · (−12) − (−20) · 8 = −80;
    // (15, 15) − S = (15, −5): −100 + 300 = 200; (10, 10) − S = (10, −10): −200 + 200 = 0.
    expect(arcChordSide(FILLET, { x: 8, y: 8 })).toBe(-80);
    expect(arcChordSide(FILLET, { x: 15, y: 15 })).toBe(200);
    expect(arcChordSide(FILLET, { x: 10, y: 10 })).toBe(0);
  });

  it("is the same for the arc travelled the other way", () => {
    // From (20, 0) to (0, 20): (−20, 20) × (−12, 8) = −160 + 240 = 80, σ = −1: −80.
    expect(arcChordSide(REVERSED, { x: 8, y: 8 })).toBe(-80);
  });
});

describe("arcChordSide (properties)", () => {
  test.prop({ x: COORDINATE, y: COORDINATE })(
    "is 20 (x + y − 20) for the fillet, the same travelled back",
    (point) => {
      // (20, −20) × (x, y − 20) = 20 (y − 20) + 20 x; travelled back, σ = −1 times
      // (−20, 20) × (x − 20, y) = −20 y − 20 (x − 20), the same number.
      const expected = 20 * (point.x + point.y - 20);

      expect(arcChordSide(FILLET, point)).toBeCloseTo(expected, 9);
      expect(arcChordSide(REVERSED, point)).toBeCloseTo(expected, 9);
    },
  );
});
