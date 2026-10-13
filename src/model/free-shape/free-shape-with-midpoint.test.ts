import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { effectiveRadii } from "../../geometry";
import { rectangleAsFreeShape } from "../rectangle";
import { regularPolygonAsFreeShape } from "../regular-polygon";

import { freeShapeCorners } from "./free-shape-corners";
import { freeShapeWithMidpoint } from "./free-shape-with-midpoint";

describe("freeShapeWithMidpoint", () => {
  it("[F08.AC3] adds a sharp point in the middle of the top edge of a square, after point 1", () => {
    const square = rectangleAsFreeShape({
      angle: 0,
      x: 0,
      y: 0,
      height: 100,
      radii: [1000, 0, 0, 0],
      width: 100,
    });
    const corners = freeShapeCorners(freeShapeWithMidpoint(square, 0));

    expect(corners).toEqual([
      { point: { x: 0, y: 0 }, radius: 1000 },
      { point: { x: 50, y: 0 }, radius: 0 },
      { point: { x: 100, y: 0 }, radius: 0 },
      { point: { x: 100, y: 100 }, radius: 0 },
      { point: { x: 0, y: 100 }, radius: 0 },
    ]);

    // Radius 1000 drawn 100 before (edges of 100), 50 after: the top edge from point 1 is now 50
    // long, its whole length for the one fillet at a right angle (DERIV-local-radius-clamp).
    expect(effectiveRadii(freeShapeCorners(square))[0]).toBeCloseTo(100, 9);
    expect(effectiveRadii(corners)[0]).toBeCloseTo(50, 9);
  });

  it("[F08.AC3] adds the point of the closing edge at the end: point 1 stays point 1", () => {
    const square = rectangleAsFreeShape({
      angle: 0,
      x: 0,
      y: 0,
      height: 100,
      radii: [0, 0, 0, 0],
      width: 100,
    });
    const points = freeShapeCorners(freeShapeWithMidpoint(square, 3)).map((corner) => corner.point);

    expect(points).toEqual([
      { x: 0, y: 0 },
      { x: 100, y: 0 },
      { x: 100, y: 100 },
      { x: 0, y: 100 },
      { x: 0, y: 50 },
    ]);
  });

  it("[F08.AC3] keeps fractions: the middle of a hexagon's base, its box unchanged", () => {
    // Hexagon in 100 × 100: the base runs from point 2 (25, 93.30127) to point 1 (75, 93.30127);
    // edge 0 goes from point 1 to point 2, its middle is (50, 50 + 25√3).
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
    const shape = freeShapeWithMidpoint(hexagon, 0);
    const [, middle] = freeShapeCorners(shape).map((corner) => corner.point);

    expect(middle?.x).toBeCloseTo(50, 9);
    expect(middle?.y).toBeCloseTo(50 + 25 * Math.sqrt(3), 9);
    expect(shape.width).toBe(hexagon.width);
    expect(shape.height).toBe(hexagon.height);
  });

  const side = fc.integer({ max: 10_000, min: 1 });

  test.prop({ edge: fc.nat({ max: 3 }), height: side, width: side })(
    "[F08.AC3] adds one point on the edge, the others and the box kept",
    ({ edge, height, width }) => {
      const shape = rectangleAsFreeShape({
        angle: 0,
        x: 0,
        y: 0,
        height,
        radii: [1, 2, 3, 4],
        width,
      });
      const before = freeShapeCorners(shape);
      const after = freeShapeCorners(freeShapeWithMidpoint(shape, edge));
      const added = after[edge + 1];
      const start = before[edge]?.point ?? { x: NaN, y: NaN };
      const end = before[(edge + 1) % 4]?.point ?? { x: NaN, y: NaN };

      expect(after.toSpliced(edge + 1, 1)).toEqual(before);

      expect(added).toEqual({
        point: { x: (start.x + end.x) / 2, y: (start.y + end.y) / 2 },
        radius: 0,
      });
    },
  );
});
