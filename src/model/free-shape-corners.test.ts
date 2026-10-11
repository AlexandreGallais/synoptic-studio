import { describe, expect, it } from "vitest";

import { cornersWithRadii } from "./corners-with-radii";
import { freeShapeCorners } from "./free-shape-corners";
import { rectangleAsFreeShape } from "./rectangle-as-free-shape";

import type { FreeShape } from "./free-shape";
import type { Point } from "../math";

/** Points and radii side by side, as F09 stored them. */
type SideBySide = {
  /** Height drawn. */
  readonly height: number;
  /** Points as stored. */
  readonly points: readonly Point[];
  /** Radius of each point. */
  readonly radii: readonly number[];
  /** Width drawn. */
  readonly width: number;
};

/**
 * Free shape from its points and radii side by side, as the tests write them (ADR-0033 keeps
 * each radius with its point).
 *
 * @param shape - points, radii, width and height
 * @returns the free shape of corners
 */
function shapeOf(shape: SideBySide): FreeShape {
  return {
    angle: 0,
    corners: cornersWithRadii(shape.points, shape.radii),
    height: shape.height,
    width: shape.width,
    x: Math.min(...shape.points.map((point) => point.x)),
    y: Math.min(...shape.points.map((point) => point.y)),
  };
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
    ).toEqual([
      { point: { x: 0, y: 0 }, radius: 10 },
      { point: { x: 100, y: 0 }, radius: 0 },
      { point: { x: 100, y: 50 }, radius: 5 },
      { point: { x: 0, y: 50 }, radius: 0 },
    ]);
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

    expect(freeShapeCorners(shapeOf(triangle)).map((corner) => corner.point)).toEqual([
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

    expect(freeShapeCorners(shapeOf(stored)).map((corner) => corner.point)).toEqual([
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
    ).toEqual([
      { point: { x: 0, y: 0 }, radius: 10 },
      { point: { x: 1, y: 0 }, radius: 0 },
      { point: { x: 1, y: 1 }, radius: 5 },
      { point: { x: 0, y: 1 }, radius: 0 },
    ]);
  });
});
