import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { boundingBox } from "../../geometry";
import { freeShapeCorners } from "../free-shape";
import { rectangleAsFreeShape } from "../rectangle";
import { regularPolygonCorners } from "../regular-polygon";

import { turnedCorners } from "./turned-corners";

import type { Rectangle } from "../rectangle";

/**
 * Drawn corners of the rectangle tool's free shape.
 *
 * @param rectangle - rectangle tool parameters
 * @returns its corners turned by its angle
 */
function drawnRectangle(rectangle: Rectangle): ReturnType<typeof turnedCorners> {
  const shape = rectangleAsFreeShape(rectangle);

  return turnedCorners(freeShapeCorners(shape), shape);
}

/** A 100 × 50 rectangle at (0, 0) with radii 1, 2, 3, 4. */
const RECTANGLE = { angle: 0, height: 50, radii: [1, 2, 3, 4], width: 100, x: 0, y: 0 };

describe("turnedCorners (DERIV-turned-shape step 4)", () => {
  it("[F04.AC1] turns a 100 × 50 rectangle by 90° about (50, 25), clockwise, exactly", () => {
    // DERIV-turned-shape checks: (0, 0) ↦ (75, −25), (100, 0) ↦ (75, 75), (100, 50) ↦ (25, 75),
    // (0, 50) ↦ (25, −25).
    expect(drawnRectangle({ ...RECTANGLE, angle: 90 })).toMatchObject([
      { point: { x: 75, y: -25 }, radius: 1 },
      { point: { x: 75, y: 75 }, radius: 2 },
      { point: { x: 25, y: 75 }, radius: 3 },
      { point: { x: 25, y: -25 }, radius: 4 },
    ]);
  });

  it.each([
    // 180°: p ↦ 2c − p, c = (50, 25).
    {
      angle: 180,
      points: [
        { x: 100, y: 50 },
        { x: 0, y: 50 },
        { x: 0, y: 0 },
        { x: 100, y: 0 },
      ],
    },
    // 270°: (dx, dy) ↦ (dy, −dx); (−50, −25) ↦ (−25, 50) ↦ (25, 75).
    {
      angle: 270,
      points: [
        { x: 25, y: 75 },
        { x: 25, y: -25 },
        { x: 75, y: -25 },
        { x: 75, y: 75 },
      ],
    },
  ])("[F04.AC1] turns it by $angle° without rounding dust", ({ angle, points }) => {
    const drawn = drawnRectangle({ ...RECTANGLE, angle });

    for (const [index, corner] of drawn.entries()) {
      expect(Object.is(corner.point.x, points[index]?.x)).toBe(true);
      expect(Object.is(corner.point.y, points[index]?.y)).toBe(true);
    }
  });

  it("[F04.AC1] gives an odd-sized rectangle back exactly after four quarter turns", () => {
    // 101 × 51: the center (50.5, 25.5) is fractional, every product stays exact.
    const shape = rectangleAsFreeShape({ ...RECTANGLE, angle: 90, height: 51, width: 101 });
    const straight = freeShapeCorners(shape);
    const turned = [1, 2, 3, 4].reduce((corners) => turnedCorners(corners, shape), straight);

    expect(turned).toEqual(straight);
  });

  it("[F04.AC3] turns about the center of the box at its position", () => {
    // At (10, 20) the center is (60, 45): (10, 20) ↦ (60 + 25, 45 − 50) = (85, −5).
    expect(drawnRectangle({ ...RECTANGLE, angle: 90, x: 10, y: 20 })[0]?.point).toEqual({
      x: 85,
      y: -5,
    });
  });

  it("[F04.AC2] gives the corners back as they are at 0°", () => {
    const shape = rectangleAsFreeShape(RECTANGLE);
    const corners = freeShapeCorners(shape);

    expect(turnedCorners(corners, shape)).toBe(corners);
  });

  it("[F04.AC2] turns a polygon's box with its content: a triangle's base at the bottom goes left at 90°", () => {
    // Anchored at the bottom of a 100 × 100 box, the base is at y = 100; turned by 90° about
    // (50, 50), (x, 100) ↦ (50 − 50, 50 + (x − 50)) = (0, x): the base is on the left.
    const triangle = {
      anchor: { horizontal: "mid", vertical: "max" },
      stretched: false,
      angle: 90,
      corners: 3,
      height: 100,
      radii: [0, 0, 0],
      width: 100,
      x: 0,
      y: 0,
    } as const;
    const drawn = turnedCorners(regularPolygonCorners(triangle), triangle);

    // Points 1 and 2 are the ends of the base (Q18 amended).
    expect(drawn[0]?.point.x).toBeCloseTo(0, 12);
    expect(drawn[1]?.point.x).toBeCloseTo(0, 12);
  });

  test.prop({
    angle: fc.integer({ max: 359, min: 0 }),
    height: fc.integer({ max: 1000, min: 0 }),
    width: fc.integer({ max: 1000, min: 0 }),
    x: fc.integer({ max: 1000, min: -1000 }),
  })(
    "[F04.AC2] keeps the radii, the order and the center of the box",
    ({ angle, height, width, x }) => {
      const drawn = drawnRectangle({ ...RECTANGLE, angle, height, width, x });
      const box = boundingBox(drawn.map((corner) => corner.point));

      expect(drawn.map((corner) => corner.radius)).toEqual([1, 2, 3, 4]);
      // A rectangle's straight box, turned or not, is centered on the center of its box.
      expect((box.minX + box.maxX) / 2).toBeCloseTo(x + width / 2, 9);
      expect((box.minY + box.maxY) / 2).toBeCloseTo(height / 2, 9);
    },
  );
});
