import { describe, expect, it } from "vitest";

import { freeShapeCorners } from "./free-shape-corners";
import { freeShapeWithPoint } from "./free-shape-with-point";
import { rectangleAsFreeShape } from "./rectangle-as-free-shape";
import { regularPolygonAsFreeShape } from "./regular-polygon-as-free-shape";

describe("freeShapeWithPoint", () => {
  it("[F08.AC2] moves a point of the rectangle to the x and y typed, the others kept, the box around them", () => {
    // 100 × 50, point 3 (index 2, bottom-right) typed at (120, 60): the box becomes 120 × 60.
    const shape = freeShapeWithPoint(
      rectangleAsFreeShape({ height: 50, radii: [10, 0, 0, 0], width: 100 }),
      2,
      { x: 120, y: 60 },
    );

    expect(shape.width).toBe(120);
    expect(shape.height).toBe(60);

    expect(freeShapeCorners(shape)).toEqual([
      { point: { x: 0, y: 0 }, radius: 10 },
      { point: { x: 100, y: 0 }, radius: 0 },
      { point: { x: 120, y: 60 }, radius: 0 },
      { point: { x: 0, y: 50 }, radius: 0 },
    ]);
  });

  it("[F08.AC2] types only x on a converted point: y keeps its fraction (Q27)", () => {
    // Hexagon in 100 × 100: point 1 is (75, 93.30127); x typed 80, y kept.
    const hexagon = regularPolygonAsFreeShape({
      anchor: { horizontal: "mid", vertical: "mid" },
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
      rectangleAsFreeShape({ height: 50, radii: [0, 0, 0, 0], width: 100 }),
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
});
