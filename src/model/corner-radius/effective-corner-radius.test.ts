import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { rectangleCorners } from "../rectangle";
import { regularPolygonCorners } from "../regular-polygon";

import { effectiveCornerRadius } from "./effective-corner-radius";

/** Decimals compared, from `EPSILON = 1e-9` (Q10). */
const DECIMALS = 9;

describe("effectiveCornerRadius (Q8, ADR-0007)", () => {
  it("[F01.AC2] keeps radius 10 on a 100 × 50 rectangle", () => {
    // S = 20 ≤ 50 and ≤ 100: no reduction.
    expect(
      effectiveCornerRadius(
        rectangleCorners({ angle: 0, x: 0, y: 0, height: 50, radii: [10, 10, 10, 10], width: 100 }),
      ),
    ).toBeCloseTo(10, DECIMALS);
  });

  it("[F01.AC2] gives 12.5 for radius 100 on a 100 × 25 rectangle, the requested 100 being kept", () => {
    // Short edges: f = 25 / 200 = 0.125 → 12.5.
    expect(
      effectiveCornerRadius(
        rectangleCorners({
          angle: 0,
          x: 0,
          y: 0,
          height: 25,
          radii: [100, 100, 100, 100],
          width: 100,
        }),
      ),
    ).toBeCloseTo(12.5, DECIMALS);
  });

  it("[F01.AC2] follows the size up to the requested value (Q8)", () => {
    // Radius 100: 100 × 300 → the 100 edges carry 200, f = 0.5 → 50; 300 × 300 → S = 200 ≤ 300 → 100.
    expect(
      effectiveCornerRadius(
        rectangleCorners({
          angle: 0,
          x: 0,
          y: 0,
          height: 300,
          radii: [100, 100, 100, 100],
          width: 100,
        }),
      ),
    ).toBeCloseTo(50, DECIMALS);

    expect(
      effectiveCornerRadius(
        rectangleCorners({
          angle: 0,
          x: 0,
          y: 0,
          height: 300,
          radii: [100, 100, 100, 100],
          width: 300,
        }),
      ),
    ).toBeCloseTo(100, DECIMALS);
  });

  test.prop({
    height: fc.integer({ max: 1000, min: 1 }),
    radius: fc.integer({ max: 2000, min: 0 }),
    width: fc.integer({ max: 999, min: 1 }),
  })(
    "never exceeds the requested radius and never shrinks when the rectangle widens",
    ({ height, radius, width }) => {
      const narrow = effectiveCornerRadius(
        rectangleCorners({
          angle: 0,
          x: 0,
          y: 0,
          height,
          radii: [radius, radius, radius, radius],
          width,
        }),
      );
      const wide = effectiveCornerRadius(
        rectangleCorners({
          angle: 0,
          x: 0,
          y: 0,
          height,
          radii: [radius, radius, radius, radius],
          width: width + 1,
        }),
      );

      expect(narrow ?? NaN).toBeLessThanOrEqual(radius * (1 + 1e-9));
      expect(wide ?? NaN).toBeGreaterThanOrEqual((narrow ?? NaN) * (1 - 1e-9));
    },
  );
});

describe("effectiveCornerRadius (stretched polygon, F06)", () => {
  it("[F06.AC5] gives no one effective radius when the corners of a stretched pentagon are clamped differently", () => {
    // Pentagon in 300 × 50 (DERIV-stretched-polygon, checks): its apex, turning 36°, keeps a larger
    // fillet than its other corners: radius 1000 is clamped to about 42.8 there, 21.8 elsewhere.
    const corners = regularPolygonCorners({
      anchor: { horizontal: "mid", vertical: "mid" },
      angle: 0,
      corners: 5,
      height: 50,
      radii: [1000, 1000, 1000, 1000, 1000],
      stretched: true,
      width: 300,
      x: 0,
      y: 0,
    });

    expect(effectiveCornerRadius(corners)).toBeUndefined();
  });

  it("[F06.AC5] gives the one radius when it fits every corner of a stretched hexagon", () => {
    const corners = regularPolygonCorners({
      anchor: { horizontal: "mid", vertical: "mid" },
      angle: 0,
      corners: 6,
      height: 50,
      radii: [5, 5, 5, 5, 5, 5],
      stretched: true,
      width: 300,
      x: 0,
      y: 0,
    });

    expect(effectiveCornerRadius(corners)).toBeCloseTo(5, DECIMALS);
  });
});
