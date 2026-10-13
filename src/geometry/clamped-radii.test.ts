import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { rectangleCorners } from "../../tests";

import { clampedRadii } from "./clamped-radii";
import { cornerTriples } from "./corner-triples";
import { turningAngles } from "./turning-angles";

/** Decimals compared: tangents and ratios of doubles, correct to their last bits. */
const DECIMALS = 9;

/** Turning angles of a rectangle drawn clockwise on screen: a quarter turn at each corner. */
const QUARTER_TURNS = [Math.PI / 2, Math.PI / 2, Math.PI / 2, Math.PI / 2];

describe("clampedRadii (DERIV-local-radius-clamp)", () => {
  it("[F01.AC3] reduces radius 1000 on every corner of a square of 100 to 50", () => {
    // s = r · tan(π/4) = 1000 at each corner, S = 2000 on every edge → f = 100 / 2000 → r′ = 50.
    const radii = clampedRadii(rectangleCorners(100, 100, [1000, 1000, 1000, 1000]), QUARTER_TURNS);

    for (const radius of radii) {
      expect(radius).toBeCloseTo(50, DECIMALS);
    }
  });

  it("clamps each corner by its own turn: radius 10 on a 3-4-5 triangle gives 1 at every corner", () => {
    // Setbacks 10 · tan(|τ|/2) = 10, 30, 20; every edge factor 0.1: r′ = 1, the inradius.
    const corners = [
      { x: 0, y: 0 },
      { x: 4, y: 0 },
      { x: 0, y: 3 },
    ].map((point) => ({ point, radius: 10 }));
    const angles = turningAngles(cornerTriples(corners.map((corner) => corner.point)));

    for (const radius of clampedRadii(corners, angles)) {
      expect(radius).toBeCloseTo(1, DECIMALS);
    }
  });

  it("keeps the radii when every corner is aligned (τ = 0): no setback to reduce", () => {
    expect(clampedRadii(rectangleCorners(0, 50, [10, 10, 10, 10]), [0, 0, 0, 0])).toEqual([
      10, 10, 10, 10,
    ]);
  });
});

/** Arbitrary side or radius. */
const SIZE = fc.integer({ max: 1000, min: 0 });

describe("clampedRadii (properties)", () => {
  test.prop({ height: SIZE, radii: fc.tuple(SIZE, SIZE, SIZE, SIZE), width: SIZE })(
    "keeps each radius between 0 and the one requested, whichever way the corners turn",
    ({ height, radii, width }) => {
      // Only |τ| enters the setbacks: negated turning angles clamp alike.
      const corners = rectangleCorners(width, height, radii);
      const clockwise = clampedRadii(corners, QUARTER_TURNS);
      const counterclockwise = clampedRadii(
        corners,
        QUARTER_TURNS.map((turn) => -turn),
      );

      for (const [index, radius] of clockwise.entries()) {
        expect(radius).toBeGreaterThanOrEqual(0);
        expect(radius).toBeLessThanOrEqual(radii[index] ?? 0);
        expect(counterclockwise[index]).toBeCloseTo(radius, DECIMALS);
      }
    },
  );
});
