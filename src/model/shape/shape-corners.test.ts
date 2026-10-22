import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { freeShapeCorners } from "../free-shape";
import { curveCornerOf } from "../handles";
import { turnedCorners } from "../placement";
import { rectangleAsFreeShape } from "../rectangle";
import { regularPolygonCorners } from "../regular-polygon";
import { roundShapeCorners } from "../round-shape";

import { shapeCorners } from "./shape-corners";

import type { RegularPolygon } from "../regular-polygon";
import type { RoundShape } from "../round-shape";

/** The rectangle a click at (250, 140) places: 100 × 100 at (200, 90), sharp (F35). */
const PLACED_RECTANGLE = rectangleAsFreeShape({
  angle: 0,
  height: 100,
  radii: [0, 0, 0, 0],
  width: 100,
  x: 200,
  y: 90,
});

/** An angle of a shape, an integer of degrees. */
const ANGLE = fc.integer({ max: 359, min: 0 });

/** A size of a box, an integer from 1. */
const SIZE = fc.integer({ max: 1000, min: 1 });

/** A coordinate of a position, an integer. */
const COORDINATE = fc.integer({ max: 1000, min: -1000 });

describe("shapeCorners (F35)", () => {
  it("draws the placed rectangle's corners from its top-left, clockwise", () => {
    // 100 × 100 at (200, 90): (200, 90), (300, 90), (300, 190), (200, 190) (Q11).
    expect(shapeCorners(PLACED_RECTANGLE).map((corner) => corner.point)).toEqual([
      { x: 200, y: 90 },
      { x: 300, y: 90 },
      { x: 300, y: 190 },
      { x: 200, y: 190 },
    ]);
  });

  it("turns them about the center of the box", () => {
    // The center is (250, 140); a quarter turn clockwise on screen takes (200, 90), 50 left and
    // 50 up of it, to 50 right and 50 up: (300, 90).
    const [first] = shapeCorners({ ...PLACED_RECTANGLE, angle: 90 });

    expect(first?.point.x).toBeCloseTo(300, 9);
    expect(first?.point.y).toBeCloseTo(90, 9);
  });

  test.prop({
    angle: ANGLE,
    corners: fc.integer({ max: 12, min: 3 }),
    height: SIZE,
    width: SIZE,
    x: COORDINATE,
    y: COORDINATE,
  })("reads a regular polygon as its straight corners, turned", (box) => {
    const polygon: RegularPolygon = {
      ...box,
      anchor: { horizontal: "mid", vertical: "mid" },
      radii: Array.from({ length: box.corners }, () => 0),
      stretched: false,
    };
    const straight = regularPolygonCorners(polygon).map((corner) => curveCornerOf(corner));

    expect(shapeCorners(polygon)).toEqual(turnedCorners(straight, polygon));
  });

  test.prop({ angle: ANGLE, height: SIZE, width: SIZE, x: COORDINATE, y: COORDINATE })(
    "reads a round shape as its four points on the curve, and a free shape as its contour, turned",
    (box) => {
      const round: RoundShape = { ...box, kind: "round" };
      const free = { ...PLACED_RECTANGLE, ...box };

      expect(shapeCorners(round)).toEqual(turnedCorners(roundShapeCorners(round), round));
      expect(shapeCorners(free)).toEqual(turnedCorners(freeShapeCorners(free), free));
    },
  );
});
