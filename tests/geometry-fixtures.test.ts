import { fc } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import {
  doubleSignedArea,
  expectPoints,
  freeShapeOf,
  integerPoint,
  integerTriangle,
  rectanglePoints,
} from "./geometry-fixtures";

describe("geometry fixtures", () => {
  it("gives the rectangle of US-001 a positive area, clockwise on screen, and its reverse a negative one", () => {
    // Shoelace sum of 100 × 50: 2 · 5000.
    expect(doubleSignedArea(rectanglePoints(100, 50))).toBe(10_000);
    expect(doubleSignedArea(rectanglePoints(100, 50).toReversed())).toBe(-10_000);
  });

  it("fails a list of points of another length or with one coordinate off", () => {
    const square = rectanglePoints(1, 1);

    expect(() => {
      expectPoints(square.slice(1), square, 9);
    }).toThrow();

    expect(() => {
      expectPoints(square, [...square.slice(0, 3), { x: 0, y: 2 }], 9);
    }).toThrow();
  });

  it("never draws three aligned points as a triangle", () => {
    // Points on one line from a generator of ±1: the filter must reject them all.
    const aligned = fc.sample(integerTriangle(1), 1000);

    expect(
      aligned.every(([a, b, c]) => (b.x - a.x) * (c.y - a.y) !== (b.y - a.y) * (c.x - a.x)),
    ).toBe(true);
  });

  it("compares to the number of decimals asked: 1e-6 off fails at 9 decimals, passes at 5", () => {
    expect(() => {
      expectPoints([{ x: 0, y: 1e-6 }], [{ x: 0, y: 0 }], 9);
    }).toThrow();

    expect(() => {
      expectPoints([{ x: 0, y: 1e-6 }], [{ x: 0, y: 0 }], 5);
    }).not.toThrow();
  });

  it("draws integer points on both sides of each axis, within the bound", () => {
    const points = fc.sample(integerPoint(5), 1000);

    expect(points.every((point) => Math.abs(point.x) <= 5 && Math.abs(point.y) <= 5)).toBe(true);
    expect(points.some((point) => point.x < 0) && points.some((point) => point.y < 0)).toBe(true);
  });

  it("draws triangles anywhere within the bound, not around one fixed vertex", () => {
    const triangles = fc.sample(integerTriangle(5), 1000);
    const firsts = new Set(triangles.map(([first]) => `${String(first.x)} ${String(first.y)}`));

    expect(firsts.size).toBeGreaterThan(1);

    expect(
      triangles.flat().every((point) => Math.abs(point.x) <= 5 && Math.abs(point.y) <= 5),
    ).toBe(true);
  });

  it("puts a free shape's box at the top-left of its points, not turned", () => {
    const shape = freeShapeOf({
      height: 20,
      points: [
        { x: 10, y: 20 },
        { x: 30, y: 20 },
        { x: 30, y: 40 },
      ],
      radii: [0, 0, 0],
      width: 20,
    });

    expect([shape.x, shape.y, shape.angle]).toEqual([10, 20, 0]);
  });
});
