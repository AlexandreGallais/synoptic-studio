import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { doubleSignedArea, expectPoints } from "../../../tests";

import { unitRegularPolygon } from "./unit-regular-polygon";

/** Decimals compared: cosine and sine of the angles are correct to the last bits of a double. */
const DECIMALS = 12;

/** Half of √2: cos(π/4), a coordinate of the square. */
const HALF_SQRT_2 = Math.SQRT2 / 2;

/** Half of √3: cos(π/6), a coordinate of the triangle and the hexagon. */
const HALF_SQRT_3 = Math.sqrt(3) / 2;

describe("unitRegularPolygon (DERIV-regular-polygon-fit steps 1–2)", () => {
  it("[F08.AC5] gives a triangle pointing up, from the right end of its base, clockwise on screen", () => {
    // n = 3, βⱼ = −π/2 + (1 − 2j) π/3: −π/6, −5π/6, −3π/2; SVG point (cos β, −sin β).
    expectPoints(
      unitRegularPolygon(3),
      [
        { x: HALF_SQRT_3, y: 0.5 },
        { x: -HALF_SQRT_3, y: 0.5 },
        { x: 0, y: -1 },
      ],
      DECIMALS,
    );
  });

  it("[F08.AC5] gives a square with horizontal edges, not a diamond, from its bottom-right vertex", () => {
    // n = 4: β = −π/4, −3π/4, −5π/4, −7π/4.
    expectPoints(
      unitRegularPolygon(4),
      [
        { x: HALF_SQRT_2, y: HALF_SQRT_2 },
        { x: -HALF_SQRT_2, y: HALF_SQRT_2 },
        { x: -HALF_SQRT_2, y: -HALF_SQRT_2 },
        { x: HALF_SQRT_2, y: -HALF_SQRT_2 },
      ],
      DECIMALS,
    );
  });

  it("[F08.AC5] gives a hexagon with flat top and bottom, from the right end of its base", () => {
    // n = 6: β = −π/3, −2π/3, −π, −4π/3, −5π/3, −2π.
    expectPoints(
      unitRegularPolygon(6),
      [
        { x: 0.5, y: HALF_SQRT_3 },
        { x: -0.5, y: HALF_SQRT_3 },
        { x: -1, y: 0 },
        { x: -0.5, y: -HALF_SQRT_3 },
        { x: 0.5, y: -HALF_SQRT_3 },
        { x: 1, y: 0 },
      ],
      DECIMALS,
    );
  });

  it("never writes a coordinate as −0 (ADR-0025)", () => {
    // No β is a multiple of π for n ≥ 3, so no coordinate is exactly 0 today; this guards later
    // formulas: y is written 0 − sin β, which gives +0 when sin β is ±0.
    for (const corners of [3, 4, 5, 6, 7, 8, 9, 10, 11, 12]) {
      const coordinates = unitRegularPolygon(corners).flatMap((point) => [point.x, point.y]);

      expect(coordinates.some((value) => Object.is(value, -0))).toBe(false);
    }
  });
});

describe("unitRegularPolygon (properties)", () => {
  test.prop({ corners: fc.integer({ max: 64, min: 3 }) })(
    "[F02.AC1] puts n vertices on the unit circle, a chord 2 sin(π/n) apart: regular",
    ({ corners }) => {
      const polygon = unitRegularPolygon(corners);
      const chord = 2 * Math.sin(Math.PI / corners);

      expect(polygon).toHaveLength(corners);

      for (const [index, point] of polygon.entries()) {
        const next = polygon[(index + 1) % corners] ?? point;

        expect(Math.hypot(point.x, point.y)).toBeCloseTo(1, DECIMALS);
        expect(Math.hypot(next.x - point.x, next.y - point.y)).toBeCloseTo(chord, DECIMALS);
      }
    },
  );

  test.prop({ corners: fc.integer({ max: 64, min: 3 }) })(
    "[F08.AC5] runs clockwise on screen from the right end of the base, then its left end",
    ({ corners }) => {
      const polygon = unitRegularPolygon(corners);
      const [right, left] = polygon;
      const bottom = Math.max(...polygon.map((point) => point.y));

      expect(doubleSignedArea(polygon)).toBeGreaterThan(0);
      expect(right?.y).toBeCloseTo(bottom, DECIMALS);
      expect(left?.y).toBeCloseTo(bottom, DECIMALS);
      expect(right?.x).toBeCloseTo(Math.sin(Math.PI / corners), DECIMALS);
      expect(left?.x).toBeCloseTo(-Math.sin(Math.PI / corners), DECIMALS);
    },
  );

  test.prop({ corners: fc.integer({ max: 64, min: 3 }) })(
    "[F02.AC1] has a flat base: its two lowest vertices are at the same height",
    ({ corners }) => {
      const heights = unitRegularPolygon(corners)
        .map((point) => point.y)
        .toSorted((first, second) => second - first);

      expect(heights[0]).toBeCloseTo(heights[1] ?? NaN, DECIMALS);
      expect(heights[0]).toBeCloseTo(Math.cos(Math.PI / corners), DECIMALS);
    },
  );
});
