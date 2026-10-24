import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { circleLevelAbscissas } from "./circle-level-abscissas";

/** The circle of radius 50 about (50, 50). */
const CIRCLE = { center: { x: 50, y: 50 }, radius: 50 };

describe("circleLevelAbscissas (DERIV-box-overlap step 7)", () => {
  it("gives the two points on a line across the circle, left first", () => {
    // h = 25 − 50 = −25, √(2500 − 625) = √1875 ≈ 43.30127: 50 ∓ 43.30127.
    const [left = NaN, right = NaN] = circleLevelAbscissas(CIRCLE, 25);

    expect(left).toBeCloseTo(6.69873, 5);
    expect(right).toBeCloseTo(93.30127, 5);
  });

  it("gives the top point twice on the tangent line", () => {
    // h = −50, √0 = 0.
    expect(circleLevelAbscissas(CIRCLE, 0)).toEqual([50, 50]);
  });

  it("gives none on a line beyond the circle", () => {
    // |h| = 60 > 50.
    expect(circleLevelAbscissas(CIRCLE, -10)).toEqual([]);
  });
});

describe("circleLevelAbscissas (properties)", () => {
  test.prop({ level: fc.double({ max: 100, min: 0, noNaN: true }) })(
    "gives points at the radius from the centre",
    ({ level }) => {
      for (const x of circleLevelAbscissas(CIRCLE, level)) {
        expect(Math.hypot(x - 50, level - 50)).toBeCloseTo(50, 6);
      }
    },
  );
});
