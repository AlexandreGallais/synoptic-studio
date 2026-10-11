import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ALIGNMENTS } from "../geometry";

import { freeShapeCorners } from "./free-shape-corners";
import { isValidFreeShape } from "./is-valid-free-shape";
import { regularPolygonAsFreeShape } from "./regular-polygon-as-free-shape";
import { regularPolygonCorners } from "./regular-polygon-corners";
import { turnedCorners } from "./turned-corners";

/** F02's placement: centered on both axes. */
const CENTER = { horizontal: "mid", vertical: "mid" } as const;

describe("regularPolygonAsFreeShape", () => {
  it("[F08.AC1] keeps a hexagon's exact corners and the size of the box around them", () => {
    const hexagon = {
      anchor: CENTER,
      angle: 0,
      x: 0,
      y: 0,
      corners: 6,
      height: 100,
      radii: [10, 0, 0, 0, 0, 0],
      width: 100,
    };
    const shape = regularPolygonAsFreeShape(hexagon);

    // 100 wide, 50√3 = 86.60254 high: fractional, as left by the conversion (Q27).
    expect(shape.width).toBeCloseTo(100, 9);
    expect(shape.height).toBeCloseTo(50 * Math.sqrt(3), 9);
    expect(shape.corners).toEqual(regularPolygonCorners(hexagon));
    expect(isValidFreeShape(shape)).toBe(true);
  });

  test.prop({
    anchor: fc.record({
      horizontal: fc.constantFrom(...ALIGNMENTS),
      vertical: fc.constantFrom(...ALIGNMENTS),
    }),
    corners: fc.integer({ max: 12, min: 3 }),
    height: fc.integer({ max: 1000, min: 1 }),
    width: fc.integer({ max: 1000, min: 1 }),
  })("[F08.AC1] draws the same corners once converted: the polygon does not move", (polygon) => {
    const radii = Array.from({ length: polygon.corners }, (unused, index) => index);
    const before = regularPolygonCorners({ angle: 0, x: 0, y: 0, ...polygon, radii });
    const after = freeShapeCorners(
      regularPolygonAsFreeShape({ angle: 0, x: 0, y: 0, ...polygon, radii }),
    );

    // At its own size a free shape is drawn as stored: the same corners, bit for bit.
    expect(after).toEqual(before);
  });

  it("gives no valid free shape for a polygon of size 0: it has no box to stretch (F08)", () => {
    const flat = regularPolygonAsFreeShape({
      angle: 0,
      x: 0,
      y: 0,
      anchor: CENTER,
      corners: 6,
      height: 100,
      radii: [0, 0, 0, 0, 0, 0],
      width: 0,
    });

    expect(isValidFreeShape(flat)).toBe(false);
  });

  it("[F04.AC4] converts a turned triangle to a free shape that keeps its angle and its drawing", () => {
    // The triangle's points do not fill its box: the free shape's box has another center, placed
    // so that the drawing does not move (DERIV-turned-shape step 5).
    const triangle = {
      anchor: { horizontal: "mid", vertical: "max" },
      angle: 90,
      corners: 3,
      height: 100,
      radii: [0, 0, 0],
      width: 100,
      x: 0,
      y: 0,
    } as const;
    const shape = regularPolygonAsFreeShape(triangle);
    const drawn = turnedCorners(freeShapeCorners(shape), shape).flatMap((corner) => [
      corner.point.x,
      corner.point.y,
    ]);
    const expected = turnedCorners(regularPolygonCorners(triangle), triangle).flatMap((corner) => [
      corner.point.x,
      corner.point.y,
    ]);

    expect(shape.angle).toBe(90);

    for (const [index, value] of drawn.entries()) {
      expect(value).toBeCloseTo(expected[index] ?? NaN, 9);
    }
  });
});
