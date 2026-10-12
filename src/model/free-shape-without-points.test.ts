import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { freeShapeCorners } from "./free-shape-corners";
import { freeShapeWithMidpoint } from "./free-shape-with-midpoint";
import { freeShapeWithoutPoints } from "./free-shape-without-points";
import { rectangleAsFreeShape } from "./rectangle-as-free-shape";

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

    expect(freeShapeCorners(shape)).toEqual([
      { point: { x: 0, y: 0 }, radius: 10 },
      { point: { x: 100, y: 100 }, radius: 30 },
      { point: { x: 0, y: 100 }, radius: 40 },
    ]);

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
});
