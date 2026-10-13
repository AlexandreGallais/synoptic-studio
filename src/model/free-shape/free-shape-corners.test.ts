import { describe, expect, it } from "vitest";

import { freeShapeOf, rectangleCorners } from "../../../tests";
import { NO_HANDLES, curveCornerOf } from "../handles";
import { rectangleAsFreeShape } from "../rectangle";

import { freeShapeAroundCorners } from "./free-shape-around-corners";
import { freeShapeCorners } from "./free-shape-corners";

import type { FreeShape } from "./free-shape";

/**
 * Radii drawn once every handle of a shape is switched on or off.
 *
 * @param shape - free shape
 * @param isOn - the switch given to every point
 * @returns the radius of each corner drawn
 */
function radiiSwitched(shape: FreeShape, isOn: boolean): readonly number[] {
  return freeShapeCorners({
    ...shape,
    corners: shape.corners.map((corner) => ({ ...corner, handles: { ...corner.handles, isOn } })),
  }).map((corner) => corner.radius);
}

describe("freeShapeCorners", () => {
  it("[F09.AC1] draws the rectangle tool's free shape as the rectangle of F01, with its radii", () => {
    expect(
      freeShapeCorners(
        rectangleAsFreeShape({
          angle: 0,
          x: 0,
          y: 0,
          height: 50,
          radii: [10, 0, 5, 0],
          width: 100,
        }),
      ),
    ).toEqual(
      [
        { point: { x: 0, y: 0 }, radius: 10 },
        { point: { x: 100, y: 0 }, radius: 0 },
        { point: { x: 100, y: 50 }, radius: 5 },
        { point: { x: 0, y: 50 }, radius: 0 },
      ].map((corner) => curveCornerOf(corner)),
    );
  });

  it("[F09.AC2] stretches any free shape, a rectangle staying a rectangle", () => {
    const triangle = {
      height: 4,
      points: [
        { x: 1, y: 0 },
        { x: 2, y: 2 },
        { x: 0, y: 2 },
      ],
      radii: [3, 0, 0],
      width: 10,
    };

    expect(freeShapeCorners(freeShapeOf(triangle)).map((corner) => corner.point)).toEqual([
      { x: 5, y: 0 },
      { x: 10, y: 4 },
      { x: 0, y: 4 },
    ]);

    // A rectangle stored 3 × 2 drawn 100 × 50: sx = 100 / 3 (multiplied first, exact edges).
    const stored = {
      height: 50,
      points: [
        { x: 0, y: 0 },
        { x: 3, y: 0 },
        { x: 3, y: 2 },
        { x: 0, y: 2 },
      ],
      radii: [0, 0, 0, 0],
      width: 100,
    };

    expect(freeShapeCorners(freeShapeOf(stored)).map((corner) => corner.point)).toEqual([
      { x: 0, y: 0 },
      { x: 100, y: 0 },
      { x: 100, y: 50 },
      { x: 0, y: 50 },
    ]);
  });

  it("[F09.AC3] gives back the same corners after a width of 0", () => {
    const rectangle = rectangleAsFreeShape({
      angle: 0,
      x: 0,
      y: 0,
      height: 50,
      radii: [10, 10, 10, 10],
      width: 100,
    });
    const flat = freeShapeCorners({ ...rectangle, width: 0 });

    expect(flat.map((corner) => corner.point.x)).toEqual([0, 0, 0, 0]);
    expect(freeShapeCorners({ ...rectangle, width: 100 })).toEqual(freeShapeCorners(rectangle));
  });

  it("[F08.AC5] keeps each radius with its point in the stored free shape", () => {
    expect(
      rectangleAsFreeShape({ angle: 0, x: 0, y: 0, height: 50, radii: [10, 0, 5, 0], width: 100 })
        .corners,
    ).toEqual(
      [
        { point: { x: 0, y: 0 }, radius: 10 },
        { point: { x: 1, y: 0 }, radius: 0 },
        { point: { x: 1, y: 1 }, radius: 5 },
        { point: { x: 0, y: 1 }, radius: 0 },
      ].map((corner) => curveCornerOf(corner)),
    );
  });
});

describe("freeShapeCorners (at a position, F04)", () => {
  it("[F04.AC3] draws the rectangle tool's shape with the top-left of its box at its position", () => {
    // The unit square stretched to 100 × 50, its top-left (0, 0) moved to (10, 20).
    const shape = rectangleAsFreeShape({
      angle: 0,
      height: 50,
      radii: [0, 0, 0, 0],
      width: 100,
      x: 10,
      y: 20,
    });

    expect(freeShapeCorners(shape).map((corner) => corner.point)).toEqual([
      { x: 10, y: 20 },
      { x: 110, y: 20 },
      { x: 110, y: 70 },
      { x: 10, y: 70 },
    ]);
  });

  it("[F12.AC2] gives the rectangle tool's corners their handles off, and stretches handles with the points", () => {
    // The unit square drawn 200 × 100: a handle (0.1, 0.2) of point 1 is drawn (20, 20).
    const rectangle = rectangleAsFreeShape({
      angle: 0,
      height: 100,
      radii: [30, 0, 0, 0],
      width: 200,
      x: 0,
      y: 0,
    });
    const handles = { ...NO_HANDLES, after: { x: 0.1, y: 0.2 }, isOn: true };
    const curved = {
      ...rectangle,
      corners: rectangle.corners.map((corner, index) =>
        index === 0 ? { ...corner, handles } : corner,
      ),
    };

    expect(rectangle.corners.map((corner) => corner.handles)).toEqual([
      NO_HANDLES,
      NO_HANDLES,
      NO_HANDLES,
      NO_HANDLES,
    ]);

    expect(freeShapeCorners(curved)[0]).toEqual({
      handles: { ...handles, after: { x: 20, y: 20 } },
      point: { x: 0, y: 0 },
      radius: 30,
    });
  });

  it("[F12.AC1] keeps each radius when the point's handles are switched on, then off", () => {
    // Radius 30 at point 1 and 0 at point 2: on, then off, they stay 30 and 0.
    const rectangle = rectangleAsFreeShape({
      angle: 0,
      height: 100,
      radii: [30, 0, 0, 0],
      width: 200,
      x: 0,
      y: 0,
    });

    expect(radiiSwitched(rectangle, true)).toEqual([30, 0, 0, 0]);
    expect(radiiSwitched(rectangle, false)).toEqual([30, 0, 0, 0]);
  });

  it("[F12.AC6] stretches other handles by the box of the curve, another point's bulge included", () => {
    // Point 1's handles (20, −20) and (−20, 20) bulge the top and left sides; point 3 has an angle handle (10, 0). At the
    // size of its own box, nothing is stretched: point 3's handle stays (10, 0).
    const square = rectangleCorners(100, 100, [0, 0, 0, 0]).map((corner, index) => {
      const handles = [
        { ...NO_HANDLES, after: { x: 20, y: -20 }, before: { x: -20, y: 20 }, isOn: true },
        NO_HANDLES,
        { ...NO_HANDLES, after: { x: 10, y: 0 }, before: { x: -10, y: 0 }, isOn: true },
        NO_HANDLES,
      ];

      return { ...corner, handles: handles[index] ?? NO_HANDLES };
    });
    const shape = freeShapeAroundCorners(square, 0);

    expect(freeShapeCorners(shape)[2]?.handles.after).toEqual({ x: 10, y: 0 });
  });
});
