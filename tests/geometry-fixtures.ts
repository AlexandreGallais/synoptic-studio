// Shapes, generators and checks shared by the tests of `src/` (EN-021): the rectangle of US-001
// as points or corners, a free shape from points and radii side by side, the shoelace area,
// integer points and triangles for properties, points compared to a number of decimals. Tests
// only: never part of the library.

import { fc } from "@fast-check/vitest";
import { expect } from "vitest";

import { cornersWithRadii } from "../src";

import type { Corner, FreeShape, Point } from "../src";

/**
 * Vertices of a `width × height` rectangle at the origin, clockwise on screen from the top-left
 * (Q11).
 *
 * @param width - horizontal side, ≥ 0
 * @param height - vertical side, ≥ 0
 * @returns its four vertices
 */
export function rectanglePoints(width: number, height: number): Point[] {
  return [
    { x: 0, y: 0 },
    { x: width, y: 0 },
    { x: width, y: height },
    { x: 0, y: height },
  ];
}

/**
 * Corners of a `width × height` rectangle at the origin, a radius at each.
 *
 * @param width - horizontal side, ≥ 0
 * @param height - vertical side, ≥ 0
 * @param radii - radius of each corner, in drawing order; a missing one is 0
 * @returns its four corners
 */
export function rectangleCorners(
  width: number,
  height: number,
  radii: readonly number[],
): Corner[] {
  return rectanglePoints(width, height).map((point, index) => ({
    point,
    radius: radii[index] ?? 0,
  }));
}

/**
 * Twice the signed area of a closed contour (shoelace formula, Σ xᵢ yᵢ₊₁ − xᵢ₊₁ yᵢ), positive
 * when clockwise on screen (y down), like the rectangle of US-001.
 *
 * @param points - vertices in drawing order
 * @returns twice the signed area
 */
export function doubleSignedArea(points: readonly Point[]): number {
  return points.reduce((sum, point, index) => {
    const next = points[(index + 1) % points.length] ?? point;

    return sum + point.x * next.y - next.x * point.y;
  }, 0);
}

/**
 * Checks a list of points against the expected one, coordinate by coordinate.
 *
 * @param actual - points computed
 * @param expected - points expected, in the same order
 * @param decimals - decimals compared
 */
export function expectPoints(
  actual: readonly Point[],
  expected: readonly Point[],
  decimals: number,
): void {
  expect(actual).toHaveLength(expected.length);

  for (const [index, point] of expected.entries()) {
    expect(actual[index]?.x).toBeCloseTo(point.x, decimals);
    expect(actual[index]?.y).toBeCloseTo(point.y, decimals);
  }
}

/**
 * Arbitrary point with integer coordinates.
 *
 * @param bound - largest absolute value of a coordinate
 * @returns the generator
 */
export function integerPoint(bound: number): fc.Arbitrary<Point> {
  return fc.record({
    x: fc.integer({ max: bound, min: -bound }),
    y: fc.integer({ max: bound, min: -bound }),
  });
}

/**
 * Arbitrary triangle of integer points with a positive area: its vertices are not aligned.
 *
 * @param bound - largest absolute value of a coordinate
 * @returns the generator of its three vertices
 */
export function integerTriangle(bound: number): fc.Arbitrary<[Point, Point, Point]> {
  const point = integerPoint(bound);

  return fc
    .tuple(point, point, point)
    .filter(([a, b, c]) => (b.x - a.x) * (c.y - a.y) !== (b.y - a.y) * (c.x - a.x));
}

/** Points and radii side by side, as F09 stored them and the tests still write them. */
export type SideBySide = {
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
 * Free shape from its points and radii side by side (ADR-0033 keeps each radius with its point),
 * not turned, its box at the top-left of its points.
 *
 * @param shape - points, radii, width and height
 * @returns the free shape of corners
 */
export function freeShapeOf(shape: SideBySide): FreeShape {
  return {
    angle: 0,
    corners: cornersWithRadii(shape.points, shape.radii),
    height: shape.height,
    width: shape.width,
    x: Math.min(...shape.points.map((point) => point.x)),
    y: Math.min(...shape.points.map((point) => point.y)),
  };
}
