import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { expectPoints } from "../../../tests";
import { boundingBox } from "../bounding-box";
import { cornerTriples, edgeLengths, turningAngles } from "../rounded-corners";

import { stretchScale } from "./stretch-scale";
import { stretchedPolygon } from "./stretched-polygon";
import { unitStretchedPolygon } from "./unit-stretched-polygon";

/** Decimals compared: the 5 of the SVG output (Q10), as the values of note 0011. */
const DECIMALS = 5;

describe("stretchedPolygon (DERIV-stretched-polygon, checks)", () => {
  it("[F06.AC2] stretches the triangle to an isosceles one and the square to the rectangle of the box", () => {
    expectPoints(
      stretchedPolygon(3, { height: 100, width: 100 }),
      [
        { x: 100, y: 100 },
        { x: 0, y: 100 },
        { x: 50, y: 0 },
      ],
      DECIMALS,
    );

    expectPoints(
      stretchedPolygon(4, { height: 50, width: 300 }),
      [
        { x: 300, y: 50 },
        { x: 0, y: 50 },
        { x: 0, y: 0 },
        { x: 300, y: 0 },
      ],
      DECIMALS,
    );
  });

  it("[F06.AC2] gives the pentagon in 100 × 100 five equal sides of 63.51246", () => {
    const pentagon = stretchedPolygon(5, { height: 100, width: 100 });

    expectPoints(
      pentagon,
      [
        { x: 81.75623, y: 100 },
        { x: 18.24377, y: 100 },
        { x: 0, y: 39.16418 },
        { x: 50, y: 0 },
        { x: 100, y: 39.16418 },
      ],
      DECIMALS,
    );

    for (const length of edgeLengths(pentagon)) {
      expect(length).toBeCloseTo(63.51246, DECIMALS);
    }
  });

  it("[F06.AC2] stops equalizing the pentagon in 300 × 50 when its apex turns 36°", () => {
    expectPoints(
      stretchedPolygon(5, { height: 50, width: 300 }),
      [
        { x: 228.85967, y: 50 },
        { x: 71.14033, y: 50 },
        { x: 0, y: 12.8595 },
        { x: 150, y: 0 },
        { x: 300, y: 12.8595 },
      ],
      DECIMALS,
    );
  });

  it("[F06.AC2] gives the hexagon in 100 × 100 six equal sides of 54.85838", () => {
    const hexagon = stretchedPolygon(6, { height: 100, width: 100 });

    expectPoints(
      hexagon,
      [
        { x: 77.42919, y: 100 },
        { x: 22.57081, y: 100 },
        { x: 0, y: 50 },
        { x: 22.57081, y: 0 },
        { x: 77.42919, y: 0 },
        { x: 100, y: 50 },
      ],
      DECIMALS,
    );

    for (const length of edgeLengths(hexagon)) {
      expect(length).toBeCloseTo(54.85838, DECIMALS);
    }
  });

  it("[F06.AC2] stretches the hexagon's limit polygon past its corner guard, in 300 × 50 and 50 × 300", () => {
    // 300 × 50: base corners at 30°, x scale 300 / (1 + √3), base end 75 + 75√3 (DERIV checks).
    expectPoints(
      stretchedPolygon(6, { height: 50, width: 300 }),
      [
        { x: 75 + 75 * Math.sqrt(3), y: 50 },
        { x: 225 - 75 * Math.sqrt(3), y: 50 },
        { x: 0, y: 25 },
        { x: 225 - 75 * Math.sqrt(3), y: 0 },
        { x: 75 + 75 * Math.sqrt(3), y: 0 },
        { x: 300, y: 25 },
      ],
      DECIMALS,
    );

    expectPoints(
      stretchedPolygon(6, { height: 300, width: 50 }),
      [
        { x: 41.47297, y: 300 },
        { x: 8.52703, y: 300 },
        { x: 0, y: 150 },
        { x: 8.52703, y: 0 },
        { x: 41.47297, y: 0 },
        { x: 50, y: 150 },
      ],
      DECIMALS,
    );
  });

  it("[F06.AC2] stops equalizing the pentagon in 100 × 300 when its side corners turn 36°", () => {
    // λ_hi = 6.74030, beyond ln λ = 1.9: the bracket [−3, 3] must hold it (review of EN-017).
    expectPoints(
      stretchedPolygon(5, { height: 300, width: 100 }),
      [
        { x: 95.60798, y: 300 },
        { x: 4.39202, y: 300 },
        { x: 0, y: 136.7215 },
        { x: 50, y: 0 },
        { x: 100, y: 136.7215 },
      ],
      DECIMALS,
    );
  });

  it("[F06.AC2] gives the octagon in 100 × 50 and the heptagon in 100 × 100 the values of note 0011", () => {
    expectPoints(
      stretchedPolygon(8, { height: 50, width: 100 }),
      [
        { x: 67.55767, y: 50 },
        { x: 32.44233, y: 50 },
        { x: 0, y: 39.16136 },
        { x: 0, y: 10.83864 },
        { x: 32.44233, y: 0 },
        { x: 67.55767, y: 0 },
        { x: 100, y: 10.83864 },
        { x: 100, y: 39.16136 },
      ],
      DECIMALS,
    );

    expectPoints(
      stretchedPolygon(7, { height: 100, width: 100 }),
      [
        { x: 72.54712, y: 100 },
        { x: 27.45288, y: 100 },
        { x: 0, y: 64.22529 },
        { x: 9.67353, y: 20.18085 },
        { x: 50, y: 0 },
        { x: 90.32647, y: 20.18085 },
        { x: 100, y: 64.22529 },
      ],
      DECIMALS,
    );
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
    fc.integer({ max: 100_000, min: 1 }),
    fc.integer({ max: 100_000, min: 1 }),
  ])(
    "[F06.AC2] touches the four sides of its box, symmetric about its vertical axis",
    (corners, width, height) => {
      const polygon = stretchedPolygon(corners, { height, width });
      const box = boundingBox(polygon);

      expect(box.minX).toBeCloseTo(0, 9);
      expect(box.minY).toBeCloseTo(0, 9);
      expect(box.maxX).toBeCloseTo(width, 9);
      expect(box.maxY).toBeCloseTo(height, 9);

      // Each point and its mirror image (the base: points 1 and 2; then k and n + 1 − k) are
      // symmetric about the vertical axis of the box, at the same height.
      for (const [index, point] of polygon.entries()) {
        const mirror = polygon[(index < 2 ? 1 : corners + 1) - index] ?? point;

        expect(point.x + mirror.x).toBeCloseTo(width, 9);
        expect(point.y).toBeCloseTo(mirror.y, 9);
      }
    },
  );

  test.prop([
    fc.integer({ max: 12, min: 5 }),
    fc.integer({ max: 100_000, min: 1 }),
    fc.integer({ max: 100_000, min: 1 }),
  ])(
    "[F06.AC2] follows rule C: its sides are equal, or its sharpest corner turns π/n",
    (corners, width, height) => {
      const lengths = edgeLengths(stretchedPolygon(corners, { height, width }));
      const unit = unitStretchedPolygon(corners, stretchScale(corners, height / width));
      const isEven = Math.max(...lengths) - Math.min(...lengths) < 1e-9 * Math.max(...lengths);
      const turns = turningAngles(cornerTriples(unit));
      const isGuarded = Math.abs(Math.min(...turns) - Math.PI / corners) < 1e-9;

      expect(isEven || isGuarded).toBe(true);
    },
  );
});
