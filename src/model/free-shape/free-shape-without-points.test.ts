import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { rectangleCorners } from "../../../tests";
import { NO_HANDLES, curveCornerOf } from "../handles";
import { rectangleAsFreeShape } from "../rectangle";

import { freeShapeAroundCorners } from "./free-shape-around-corners";
import { freeShapeCorners } from "./free-shape-corners";
import { freeShapeWithMidpoint } from "./free-shape-with-midpoint";
import { freeShapeWithoutPoints } from "./free-shape-without-points";

import type { FreeShape } from "./free-shape";

/**
 * The rectangle 200 × 100, its point 1's handle after (0.1, 0.2) in the unit square, on: (20, 20) as drawn.
 *
 * @param angle - 0 or 90 degrees, clockwise
 * @returns the free shape
 */
function curvedRectangle(angle: number): FreeShape {
  const rectangle = rectangleAsFreeShape({
    angle,
    height: 100,
    radii: [0, 0, 0, 0],
    width: 200,
    x: 0,
    y: 0,
  });

  return {
    ...rectangle,
    corners: rectangle.corners.map((corner, index) =>
      index === 0
        ? { ...corner, handles: { ...corner.handles, after: { x: 0.1, y: 0.2 }, isOn: true } }
        : corner,
    ),
  };
}

describe("freeShapeWithoutPoints", () => {
  it("[F08.AC4] removes a point and its radius: its neighbours are joined by a straight edge", () => {
    // 100 square, radii 10, 20, 30, 40; point 2 (100, 0) removed: a triangle of the three others.
    const square = rectangleAsFreeShape({
      angle: 0,
      x: 0,
      y: 0,
      height: 100,
      radii: [10, 20, 30, 40],
      width: 100,
    });
    const shape = freeShapeWithoutPoints(square, [1]);

    expect(freeShapeCorners(shape)).toEqual(
      [
        { point: { x: 0, y: 0 }, radius: 10 },
        { point: { x: 100, y: 100 }, radius: 30 },
        { point: { x: 0, y: 100 }, radius: 40 },
      ].map((corner) => curveCornerOf(corner)),
    );

    expect([shape.width, shape.height]).toEqual([100, 100]);
  });

  it("[F08.AC4] removes the first and the last points: their neighbours wrap round", () => {
    // Points (0, 0), (50, 0), (100, 0), (100, 100), (0, 100); the first and the last removed:
    // (50, 0) becomes point 1, joined to (100, 100); the box shrinks to 50 × 100 where they are.
    const five = freeShapeWithMidpoint(
      rectangleAsFreeShape({ angle: 0, x: 0, y: 0, height: 100, radii: [0, 0, 0, 0], width: 100 }),
      0,
    );
    const shape = freeShapeWithoutPoints(five, [4, 0]);

    expect(freeShapeCorners(shape).map((corner) => corner.point)).toEqual([
      { x: 50, y: 0 },
      { x: 100, y: 0 },
      { x: 100, y: 100 },
    ]);

    expect([shape.width, shape.height]).toEqual([50, 100]);
  });

  it("removes nothing for an empty list", () => {
    const square = rectangleAsFreeShape({
      angle: 0,
      x: 0,
      y: 0,
      height: 50,
      radii: [1, 2, 3, 4],
      width: 80,
    });

    expect(freeShapeCorners(freeShapeWithoutPoints(square, []))).toEqual(freeShapeCorners(square));
  });

  test.prop({ removed: fc.uniqueArray(fc.nat({ max: 4 }), { maxLength: 2 }) })(
    "[F08.AC4] keeps the other points, in order, where they are drawn",
    ({ removed }) => {
      const five = freeShapeWithMidpoint(
        rectangleAsFreeShape({ angle: 0, x: 0, y: 0, height: 70, radii: [1, 2, 3, 4], width: 90 }),
        2,
      );
      const kept = freeShapeCorners(five).filter((corner, index) => !removed.includes(index));

      expect(freeShapeCorners(freeShapeWithoutPoints(five, removed))).toEqual(kept);
    },
  );

  it.each([0, 90])("keeps the handles on, as drawn, after deleting a point (angle %i)", (angle) => {
    // Stored at scale 1 after the edit: the handle is stored and drawn (20, 20).
    const edited = freeShapeWithoutPoints(curvedRectangle(angle), [1]);

    expect(edited.corners[0]?.handles).toEqual({
      ...NO_HANDLES,
      after: { x: 20, y: 20 },
      isOn: true,
    });

    expect(freeShapeCorners(edited)[0]?.handles.after).toEqual({ x: 20, y: 20 });
  });

  it("[F12.AC3] keeps the outer handles when a point between two curved sides is deleted", () => {
    // Point 1 after (0, −40), point 2 before (0, −40) and after (40, 0), point 3 before (40, 0):
    // deleting point 2 joins points 1 and 3 by a cubic with their own handles.
    const handles = [
      { after: { x: 0, y: -40 }, before: { x: 0, y: 0 } },
      { after: { x: 40, y: 0 }, before: { x: 0, y: -40 } },
      { after: { x: 0, y: 0 }, before: { x: 40, y: 0 } },
      { after: { x: 0, y: 0 }, before: { x: 0, y: 0 } },
    ];
    const shape = freeShapeAroundCorners(
      rectangleCorners(100, 100, [0, 0, 0, 0]).map((stored, index) => ({
        ...stored,
        handles: { ...NO_HANDLES, ...handles[index], isOn: index < 3, mode: "none" as const },
      })),
      0,
    );
    const left = freeShapeWithoutPoints(shape, [1]);

    expect([left.corners[0]?.handles.after, left.corners[1]?.handles.before]).toEqual([
      { x: 0, y: -40 },
      { x: 40, y: 0 },
    ]);
  });
});
