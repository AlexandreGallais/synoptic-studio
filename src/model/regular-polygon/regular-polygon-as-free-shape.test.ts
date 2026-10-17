import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ALIGNMENTS } from "../../geometry";
import { freeShapeCorners, isValidFreeShape } from "../free-shape";
import { curveCornerOf } from "../handles";
import { turnedCorners } from "../placement";

import { regularPolygonAsFreeShape } from "./regular-polygon-as-free-shape";
import { regularPolygonCorners } from "./regular-polygon-corners";

/** F02's placement: centered on both axes. */
const CENTER = { horizontal: "mid", vertical: "mid" } as const;

describe("regularPolygonAsFreeShape", () => {
  it("[F08.AC1] keeps a hexagon's exact corners and the size of the box around them", () => {
    const hexagon = {
      anchor: CENTER,
      stretched: false,
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

    expect(shape.corners).toEqual(
      regularPolygonCorners(hexagon).map((corner) => curveCornerOf(corner)),
    );

    expect(isValidFreeShape(shape)).toBe(true);
  });

  test.prop({
    anchor: fc.record({
      horizontal: fc.constantFrom(...ALIGNMENTS),
      vertical: fc.constantFrom(...ALIGNMENTS),
    }),
    corners: fc.integer({ max: 12, min: 3 }),
    height: fc.integer({ max: 1000, min: 1 }),
    stretched: fc.boolean(),
    width: fc.integer({ max: 1000, min: 1 }),
  })(
    "[F08.AC1] [F06.AC4] draws the same corners once converted, stretched or not: the polygon does not move",
    (polygon) => {
      const radii = Array.from({ length: polygon.corners }, (unused, index) => index);
      const before = regularPolygonCorners({ angle: 0, x: 0, y: 0, ...polygon, radii });
      const after = freeShapeCorners(
        regularPolygonAsFreeShape({ angle: 0, x: 0, y: 0, ...polygon, radii }),
      );

      // At its own size a free shape is drawn as stored: the same corners, bit for bit.
      expect(after).toEqual(before.map((corner) => curveCornerOf(corner)));
    },
  );

  it("gives no valid free shape for a polygon of size 0: it has no box to stretch (F08)", () => {
    const flat = regularPolygonAsFreeShape({
      angle: 0,
      x: 0,
      y: 0,
      anchor: CENTER,
      stretched: false,
      corners: 6,
      height: 100,
      radii: [0, 0, 0, 0, 0, 0],
      width: 0,
    });

    expect(isValidFreeShape(flat)).toBe(false);
  });

  test.prop({
    angle: fc.integer({ max: 359, min: 1 }),
    corners: fc.integer({ max: 12, min: 3 }),
    horizontal: fc.constantFrom("min", "mid", "max"),
    vertical: fc.constantFrom("min", "mid", "max"),
  })(
    "[F04.AC4] converts a polygon turned by any angle, from any anchor, keeping its angle and its drawing",
    ({ angle, corners, horizontal, vertical }) => {
      // The polygon's points need not fill its box: the free shape's box may have another center,
      // placed so that the drawing does not move (DERIV-turned-shape step 5).
      const polygon = {
        anchor: { horizontal, vertical },
        stretched: false,
        angle,
        corners,
        height: 80,
        radii: Array.from({ length: corners }, () => 0),
        width: 120,
        x: 5,
        y: -7,
      } as const;
      const shape = regularPolygonAsFreeShape(polygon);
      const drawn = turnedCorners(freeShapeCorners(shape), shape).flatMap((corner) => [
        corner.point.x,
        corner.point.y,
      ]);
      const expected = turnedCorners(
        regularPolygonCorners(polygon).map((corner) => curveCornerOf(corner)),
        polygon,
      ).flatMap((corner) => [corner.point.x, corner.point.y]);

      expect(shape.angle).toBe(angle);

      for (const [index, value] of drawn.entries()) {
        expect(value).toBeCloseTo(expected[index] ?? NaN, 9);
      }
    },
  );
});
