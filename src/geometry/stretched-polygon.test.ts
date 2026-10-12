import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { boundingBox } from "./bounding-box";
import { edgeLengths } from "./edge-lengths";
import { stretchedPolygon } from "./stretched-polygon";

import type { Point } from "../math";

/** Decimals compared: the 5 of the SVG output (Q10), as the values of note 0011. */
const DECIMALS = 5;

/**
 * Checks two lists of points coordinate by coordinate, to 5 decimals.
 *
 * @param actual - computed points
 * @param expected - values of `DERIV-stretched-polygon`, checks
 */
function expectPoints(
  actual: readonly Point[],
  expected: readonly (readonly [number, number])[],
): void {
  expect(actual).toHaveLength(expected.length);

  for (const [index, [x, y]] of expected.entries()) {
    expect(actual[index]?.x).toBeCloseTo(x, DECIMALS);
    expect(actual[index]?.y).toBeCloseTo(y, DECIMALS);
  }
}

describe("stretchedPolygon (DERIV-stretched-polygon, checks)", () => {
  it("[F06.AC2] stretches the triangle to an isosceles one and the square to the rectangle of the box", () => {
    expectPoints(stretchedPolygon(3, { height: 100, width: 100 }), [
      [100, 100],
      [0, 100],
      [50, 0],
    ]);

    expectPoints(stretchedPolygon(4, { height: 50, width: 300 }), [
      [300, 50],
      [0, 50],
      [0, 0],
      [300, 0],
    ]);
  });

  it("[F06.AC2] gives the pentagon in 100 × 100 five equal sides of 63.51246", () => {
    const pentagon = stretchedPolygon(5, { height: 100, width: 100 });

    expectPoints(pentagon, [
      [81.75623, 100],
      [18.24377, 100],
      [0, 39.16418],
      [50, 0],
      [100, 39.16418],
    ]);

    for (const length of edgeLengths(pentagon)) {
      expect(length).toBeCloseTo(63.51246, DECIMALS);
    }
  });

  it("[F06.AC2] stops equalizing the pentagon in 300 × 50 when its apex turns 36°", () => {
    expectPoints(stretchedPolygon(5, { height: 50, width: 300 }), [
      [228.85967, 50],
      [71.14033, 50],
      [0, 12.8595],
      [150, 0],
      [300, 12.8595],
    ]);
  });

  it("[F06.AC2] gives the hexagon in 100 × 100 six equal sides of 54.85838", () => {
    const hexagon = stretchedPolygon(6, { height: 100, width: 100 });

    expectPoints(hexagon, [
      [77.42919, 100],
      [22.57081, 100],
      [0, 50],
      [22.57081, 0],
      [77.42919, 0],
      [100, 50],
    ]);

    for (const length of edgeLengths(hexagon)) {
      expect(length).toBeCloseTo(54.85838, DECIMALS);
    }
  });

  it("[F06.AC2] stretches the hexagon's limit polygon past its corner guard, in 300 × 50 and 50 × 300", () => {
    // 300 × 50: base corners at 30°, x scale 300 / (1 + √3), base end 75 + 75√3 (DERIV checks).
    expectPoints(stretchedPolygon(6, { height: 50, width: 300 }), [
      [75 + 75 * Math.sqrt(3), 50],
      [225 - 75 * Math.sqrt(3), 50],
      [0, 25],
      [225 - 75 * Math.sqrt(3), 0],
      [75 + 75 * Math.sqrt(3), 0],
      [300, 25],
    ]);

    expectPoints(stretchedPolygon(6, { height: 300, width: 50 }), [
      [41.47297, 300],
      [8.52703, 300],
      [0, 150],
      [8.52703, 0],
      [41.47297, 0],
      [50, 150],
    ]);
  });

  it("[F06.AC2] gives the octagon in 100 × 50 and the heptagon in 100 × 100 the values of note 0011", () => {
    expectPoints(stretchedPolygon(8, { height: 50, width: 100 }), [
      [67.55767, 50],
      [32.44233, 50],
      [0, 39.16136],
      [0, 10.83864],
      [32.44233, 0],
      [67.55767, 0],
      [100, 10.83864],
      [100, 39.16136],
    ]);

    expectPoints(stretchedPolygon(7, { height: 100, width: 100 }), [
      [72.54712, 100],
      [27.45288, 100],
      [0, 64.22529],
      [9.67353, 20.18085],
      [50, 0],
      [90.32647, 20.18085],
      [100, 64.22529],
    ]);
  });

  it("[F06.AC2] gives the regular octagon in a square box: side 100 (√2 − 1)", () => {
    const octagon = stretchedPolygon(8, { height: 100, width: 100 });

    for (const length of edgeLengths(octagon)) {
      expect(length).toBeCloseTo(100 * (Math.SQRT2 - 1), DECIMALS);
    }
  });

  it("[F06.AC2] flattens a polygon in a box of size 0 onto the side at 0", () => {
    const flat = stretchedPolygon(6, { height: 40, width: 0 });

    expect(flat.every((point) => point.x === 0)).toBe(true);

    expect(
      stretchedPolygon(5, { height: 0, width: 0 }).every((point) => point.x === 0 && point.y === 0),
    ).toBe(true);
  });

  test.prop([
    fc.integer({ max: 12, min: 3 }),
    fc.integer({ max: 1000, min: 1 }),
    fc.integer({ max: 1000, min: 1 }),
  ])(
    "[F06.AC2] touches the four sides of its box, symmetric about its vertical axis",
    (corners, width, height) => {
      const polygon = stretchedPolygon(corners, { height, width });
      const box = boundingBox(polygon);

      expect(box.minX).toBeCloseTo(0, 9);
      expect(box.minY).toBeCloseTo(0, 9);
      expect(box.maxX).toBeCloseTo(width, 9);
      expect(box.maxY).toBeCloseTo(height, 9);

      // The base (points 1 and 2) is flat and centered.
      const base = polygon.slice(0, 2);

      expect(base.every((point) => Math.abs(point.y - height) < 1e-9)).toBe(true);
      expect(base.reduce((sum, point) => sum + point.x, 0)).toBeCloseTo(width, 9);
    },
  );
});
