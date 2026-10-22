import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isInCircularSegment } from "./is-in-circular-segment";

import type { Arc } from "../contour";

/**
 * The fillet of the top-left corner of a box 100 × 100 of radius 20, the contour clockwise on
 * screen: from (0, 20) up to (20, 0) about (20, 20), the angle growing from π to 3π/2.
 */
const FILLET: Arc = {
  center: { x: 20, y: 20 },
  end: { x: 20, y: 0 },
  kind: "arc",
  positive: true,
  radius: 20,
  start: { x: 0, y: 20 },
};

/** The same fillet travelled the other way. */
const REVERSED: Arc = { ...FILLET, end: FILLET.start, positive: false, start: FILLET.end };

/** A coordinate, an integer, so that the side test is exact. */
const COORDINATE = fc.integer({ max: 40, min: -20 });

describe("isInCircularSegment (DERIV-painted-area step 3)", () => {
  it("holds for (8, 8), between the arc and its chord", () => {
    // |P − C| = 12√2 ≈ 16.97 < 20; (20, −20) × (8, −12) = −240 + 160 = −80, σ = +1: −80 < 0.
    expect(isInCircularSegment(FILLET, { x: 8, y: 8 })).toBe(true);
  });

  it("fails for (3, 3), in the corner cut off, outside the circle", () => {
    // |P − C| = 17√2 ≈ 24.04 > 20.
    expect(isInCircularSegment(FILLET, { x: 3, y: 3 })).toBe(false);
  });

  it("fails for (15, 15), in the circle on the centre's side of the chord", () => {
    // (20, −20) × (15, −5) = −100 + 300 = 200 > 0.
    expect(isInCircularSegment(FILLET, { x: 15, y: 15 })).toBe(false);
  });

  it("fails on the chord itself", () => {
    // (20, −20) × (10, −10) = −200 + 200 = 0.
    expect(isInCircularSegment(FILLET, { x: 10, y: 10 })).toBe(false);
  });

  it("holds for (8, 8) with the arc travelled the other way", () => {
    // From (20, 0) to (0, 20): (−20, 20) × (−12, 8) = −160 + 240 = 80, σ = −1: −80 < 0.
    expect(isInCircularSegment(REVERSED, { x: 8, y: 8 })).toBe(true);
  });
});

describe("isInCircularSegment (properties)", () => {
  test.prop({ x: COORDINATE, y: COORDINATE })(
    "is the same for the arc travelled the other way",
    (point) => {
      expect(isInCircularSegment(REVERSED, point)).toBe(isInCircularSegment(FILLET, point));
    },
  );

  test.prop({ x: COORDINATE, y: COORDINATE })(
    "holds only inside the circle and beyond the chord, x + y < 20",
    (point) => {
      // The chord is the line x + y = 20; the circle the points 20 from (20, 20).
      const isInside = (point.x - 20) ** 2 + (point.y - 20) ** 2 < 400 && point.x + point.y < 20;

      expect(isInCircularSegment(FILLET, point)).toBe(isInside);
    },
  );
});
