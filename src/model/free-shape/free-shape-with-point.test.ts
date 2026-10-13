import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { NO_HANDLES, curveCornerOf } from "../handles";
import { rectangleAsFreeShape } from "../rectangle";
import { regularPolygonAsFreeShape } from "../regular-polygon";

import { freeShapeCorners } from "./free-shape-corners";
import { freeShapeWithPoint } from "./free-shape-with-point";
import { isValidFreeShape } from "./is-valid-free-shape";

import type { FreeShape } from "./free-shape";

/**
 * The rectangle 200 × 100, its point 1's handle after (0.1, −0.2) in the unit square, on: (20, −20) as drawn.
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
        ? { ...corner, handles: { ...corner.handles, after: { x: 0.1, y: -0.2 }, isOn: true } }
        : corner,
    ),
  };
}

describe("freeShapeWithPoint", () => {
  it("[F08.AC2] moves a point of the rectangle to the x and y typed, the others kept, the box around them", () => {
    // 100 × 50, point 3 (index 2, bottom-right) typed at (120, 60): the box becomes 120 × 60.
    const shape = freeShapeWithPoint(
      rectangleAsFreeShape({ angle: 0, x: 0, y: 0, height: 50, radii: [10, 0, 0, 0], width: 100 }),
      2,
      { x: 120, y: 60 },
    );

    expect(shape.width).toBe(120);
    expect(shape.height).toBe(60);

    expect(freeShapeCorners(shape)).toEqual(
      [
        { point: { x: 0, y: 0 }, radius: 10 },
        { point: { x: 100, y: 0 }, radius: 0 },
        { point: { x: 120, y: 60 }, radius: 0 },
        { point: { x: 0, y: 50 }, radius: 0 },
      ].map((corner) => curveCornerOf(corner)),
    );
  });

  it("[F08.AC2] types only x on a converted point: y keeps its fraction (Q27)", () => {
    // Hexagon in 100 × 100: point 1 is (75, 93.30127); x typed 80, y kept.
    const hexagon = regularPolygonAsFreeShape({
      angle: 0,
      x: 0,
      y: 0,
      anchor: { horizontal: "mid", vertical: "mid" },
      stretched: false,
      corners: 6,
      height: 100,
      radii: [0, 0, 0, 0, 0, 0],
      width: 100,
    });
    const [first = { point: { x: NaN, y: NaN }, radius: 0 }] = freeShapeCorners(hexagon);
    const moved = freeShapeCorners(freeShapeWithPoint(hexagon, 0, { x: 80, y: first.point.y }));
    const [point1, , point3] = moved.map((corner) => corner.point);

    expect(point1).toEqual({ x: 80, y: first.point.y });
    expect(first.point.y).toBeCloseTo(50 + 25 * Math.sqrt(3), 9);
    expect(point3?.x).toBeCloseTo(0, 9);
  });

  it("[F08.AC2] keeps the shape stretchable without loss after a point is typed", () => {
    const shape = freeShapeWithPoint(
      rectangleAsFreeShape({ angle: 0, x: 0, y: 0, height: 50, radii: [0, 0, 0, 0], width: 100 }),
      2,
      {
        x: 120,
        y: 60,
      },
    );

    expect(freeShapeCorners({ ...shape, width: 0 }).map((corner) => corner.point.x)).toEqual([
      0, 0, 0, 0,
    ]);

    expect(freeShapeCorners({ ...shape, width: 120 })).toEqual(freeShapeCorners(shape));
  });

  it("refuses, through its validity, a point that puts every point on one line", () => {
    const flat = freeShapeWithPoint(
      rectangleAsFreeShape({ angle: 0, x: 0, y: 0, height: 50, radii: [0, 0, 0, 0], width: 0 }),
      2,
      { x: 0, y: 60 },
    );

    expect(isValidFreeShape(flat)).toBe(false);
  });

  it("accepts a point typed onto another one, or far to the left and above", () => {
    const rectangle = rectangleAsFreeShape({
      angle: 0,
      x: 0,
      y: 0,
      height: 50,
      radii: [0, 0, 0, 0],
      width: 100,
    });

    expect(isValidFreeShape(freeShapeWithPoint(rectangle, 2, { x: 100, y: 0 }))).toBe(true);

    const far = freeShapeWithPoint(rectangle, 0, { x: -1_000_000, y: -5 });

    expect(freeShapeCorners(far).map((corner) => corner.point)).toEqual([
      { x: -1_000_000, y: -5 },
      { x: 100, y: 0 },
      { x: 100, y: 50 },
      { x: 0, y: 50 },
    ]);
  });

  it("[F08.AC5] moves a point with its radius", () => {
    // Point 1 of a 100 × 50 rectangle, radius 10, moved to (-20, -10): it keeps its radius 10.
    const shape = freeShapeWithPoint(
      rectangleAsFreeShape({ angle: 0, x: 0, y: 0, height: 50, radii: [10, 0, 0, 0], width: 100 }),
      0,
      { x: -20, y: -10 },
    );

    expect(freeShapeCorners(shape)[0]).toEqual({
      handles: NO_HANDLES,
      point: { x: -20, y: -10 },
      radius: 10,
    });
  });

  const coordinate = fc.integer({ max: 1000, min: -1000 });

  test.prop({ index: fc.nat({ max: 3 }), x: coordinate, y: coordinate })(
    "[F08.AC2] puts the point where it is typed, the others unchanged, the box around them",
    ({ index, x, y }) => {
      const rectangle = rectangleAsFreeShape({
        angle: 0,
        x: 0,
        y: 0,
        height: 50,
        radii: [1, 2, 3, 4],
        width: 100,
      });
      const before = freeShapeCorners(rectangle);
      const shape = freeShapeWithPoint(rectangle, index, { x, y });
      const after = freeShapeCorners(shape);
      const xs = after.map((corner) => corner.point.x);
      const ys = after.map((corner) => corner.point.y);

      expect(after.toSpliced(index, 1)).toEqual(before.toSpliced(index, 1));
      expect(after[index]).toEqual({ handles: NO_HANDLES, point: { x, y }, radius: index + 1 });

      expect([shape.width, shape.height]).toEqual([
        Math.max(...xs) - Math.min(...xs),
        Math.max(...ys) - Math.min(...ys),
      ]);
    },
  );

  it.each([0, 90])("keeps the handles on, as drawn, after moving a point (angle %i)", (angle) => {
    // Stored at scale 1 after the edit: the handle is stored and drawn (20, −20).
    const edited = freeShapeWithPoint(curvedRectangle(angle), 2, { x: 250, y: 100 });

    expect(edited.corners[0]?.handles).toEqual({
      ...NO_HANDLES,
      after: { x: 20, y: -20 },
      isOn: true,
    });

    expect(freeShapeCorners(edited)[0]?.handles.after).toEqual({ x: 20, y: -20 });
  });
});
