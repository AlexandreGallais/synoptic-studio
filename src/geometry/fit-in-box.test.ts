import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { EPSILON } from "../math";

import { boundingBox } from "./bounding-box";
import { fitInBox } from "./fit-in-box";
import { unitRegularPolygon } from "./unit-regular-polygon";

import type { Alignment } from "./alignment";
import type { Anchor } from "./anchor";
import type { Point } from "../math";

/** Unit square from (−1, −1) to (1, 1), clockwise on screen from its top-left vertex. */
const SQUARE = [
  { x: -1, y: -1 },
  { x: 1, y: -1 },
  { x: 1, y: 1 },
  { x: -1, y: 1 },
];

/** F02's placement: centered on both axes. */
const CENTER: Anchor = { horizontal: "mid", vertical: "mid" };

/** Every alignment, for properties. */
const ALIGNMENT = fc.constantFrom<Alignment>("min", "mid", "max");

/** √3 / 2 · 100 = 86.60254…: height of the triangle fitted in 100 × 100. */
const HEIGHT_3 = 50 * Math.sqrt(3);

/**
 * Checks two lists of points coordinate by coordinate, to 9 decimals (more than the 5 written).
 *
 * @param actual - computed points
 * @param expected - hand-computed points
 */
function expectPoints(actual: readonly Point[], expected: readonly Point[]): void {
  expect(actual).toHaveLength(expected.length);

  for (const [index, point] of expected.entries()) {
    expect(actual[index]?.x).toBeCloseTo(point.x, 9);
    expect(actual[index]?.y).toBeCloseTo(point.y, 9);
  }
}

/**
 * Largest distance, on either axis, between the points of two lists of the same length.
 *
 * @param moved - points after a change
 * @param reference - the same points before it
 * @returns the largest coordinate difference
 */
function largestMove(moved: readonly Point[], reference: readonly Point[]): number {
  return Math.max(
    ...moved.map((point, index) => {
      const before = reference[index] ?? point;

      return Math.max(Math.abs(point.x - before.x), Math.abs(point.y - before.y));
    }),
  );
}

describe("fitInBox (DERIV-regular-polygon-fit steps 3–5)", () => {
  it("fits a square in 100 × 50: the height is reached, centered horizontally", () => {
    // Span 2 × 2, s = min(100/2, 50/2) = 25: a 50 × 50 square, left margin (100 − 50)/2 = 25.
    expect(fitInBox(SQUARE, { height: 50, width: 100 }, CENTER)).toEqual([
      { x: 25, y: 0 },
      { x: 75, y: 0 },
      { x: 75, y: 50 },
      { x: 25, y: 50 },
    ]);
  });

  it("fits a 4 × 2 rectangle in 100 × 100: the width is reached, centered vertically", () => {
    // Span 4 × 2, s = min(100/4, 100/2) = 25: 100 × 50, top margin (100 − 50)/2 = 25.
    const rectangle = [
      { x: 0, y: 0 },
      { x: 4, y: 0 },
      { x: 4, y: 2 },
      { x: 0, y: 2 },
    ];

    expect(fitInBox(rectangle, { height: 100, width: 100 }, CENTER)).toEqual([
      { x: 0, y: 25 },
      { x: 100, y: 25 },
      { x: 100, y: 75 },
      { x: 0, y: 75 },
    ]);
  });

  it("collapses every point to the center of a box of width 0 (Q15)", () => {
    // s = min(0/2, 50/2) = 0: every point at (0, 25).
    expect(fitInBox(SQUARE, { height: 50, width: 0 }, CENTER)).toEqual(
      Array.from({ length: 4 }, () => ({ x: 0, y: 25 })),
    );
  });

  it("[F07.AC2] puts the square on the left or the right of 100 × 50", () => {
    // Room Δx = 100 − 50 = 50: none before at min, all of it at max.
    expect(
      fitInBox(SQUARE, { height: 50, width: 100 }, { horizontal: "min", vertical: "mid" }),
    ).toEqual([
      { x: 0, y: 0 },
      { x: 50, y: 0 },
      { x: 50, y: 50 },
      { x: 0, y: 50 },
    ]);

    expect(
      fitInBox(SQUARE, { height: 50, width: 100 }, { horizontal: "max", vertical: "mid" }),
    ).toEqual([
      { x: 50, y: 0 },
      { x: 100, y: 0 },
      { x: 100, y: 50 },
      { x: 50, y: 50 },
    ]);
  });

  it("[F07.AC2] puts the triangle on the bottom or the top of 100 × 100", () => {
    // Room Δy = 100 − 86.60254 = 13.39746: all before the shape at max, none at min.
    const triangle = unitRegularPolygon(3);
    const room = 100 - HEIGHT_3;

    expectPoints(
      fitInBox(triangle, { height: 100, width: 100 }, { horizontal: "mid", vertical: "max" }),
      [
        { x: 50, y: room },
        { x: 100, y: 100 },
        { x: 0, y: 100 },
      ],
    );

    expectPoints(
      fitInBox(triangle, { height: 100, width: 100 }, { horizontal: "mid", vertical: "min" }),
      [
        { x: 50, y: 0 },
        { x: 100, y: HEIGHT_3 },
        { x: 0, y: HEIGHT_3 },
      ],
    );
  });

  it("[F07.AC3] keeps the triangle in place when only its horizontal alignment changes", () => {
    // The width is reached: Δx = 0, whatever the share of it before the shape.
    const triangle = unitRegularPolygon(3);
    const size = { height: 100, width: 100 };
    const centered = fitInBox(triangle, size, { horizontal: "mid", vertical: "max" });

    expectPoints(fitInBox(triangle, size, { horizontal: "min", vertical: "max" }), centered);
    expectPoints(fitInBox(triangle, size, { horizontal: "max", vertical: "max" }), centered);
  });
});

describe("fitInBox (properties)", () => {
  test.prop({
    height: fc.integer({ max: 1000, min: 0 }),
    horizontal: ALIGNMENT,
    stretch: fc.integer({ max: 10, min: 1 }),
    vertical: ALIGNMENT,
    width: fc.integer({ max: 1000, min: 0 }),
  })(
    "[F07.AC2] stays in the box, on the anchored side, reaching its width or its height",
    ({ height, horizontal, stretch, vertical, width }) => {
      const rectangle = SQUARE.map((point) => ({ x: point.x * stretch, y: point.y }));
      const box = boundingBox(fitInBox(rectangle, { height, width }, { horizontal, vertical }));
      const tolerance = EPSILON * Math.max(1, width, height);
      const sides = [
        { alignment: horizontal, high: box.maxX, low: box.minX, size: width },
        { alignment: vertical, high: box.maxY, low: box.minY, size: height },
      ];

      for (const { alignment, high, low, size } of sides) {
        expect(low).toBeGreaterThanOrEqual(-tolerance);
        expect(high).toBeLessThanOrEqual(size + tolerance);

        const gap = { max: size - high, mid: (size - high - low) / 2, min: low }[alignment];

        expect(Math.abs(gap)).toBeLessThan(tolerance);
      }

      expect(
        Math.abs(box.maxX - box.minX - width) < tolerance ||
          Math.abs(box.maxY - box.minY - height) < tolerance,
      ).toBe(true);
    },
  );

  test.prop({
    corners: fc.integer({ max: 12, min: 3 }),
    height: fc.integer({ max: 1000, min: 0 }),
    horizontal: ALIGNMENT,
    vertical: ALIGNMENT,
    width: fc.integer({ max: 1000, min: 0 }),
  })(
    "[F07.AC3] moves a polygon by a rounding error at most along the axis it fills",
    ({ corners, height, horizontal, vertical, width }) => {
      const polygon = unitRegularPolygon(corners);
      const size = { height, width };
      const centered = fitInBox(polygon, size, CENTER);
      const box = boundingBox(centered);
      const isWidthFilled = Math.abs(box.maxX - box.minX - width) < EPSILON * Math.max(1, width);
      const anchor: Anchor = isWidthFilled
        ? { horizontal, vertical: "mid" }
        : { horizontal: "mid", vertical };

      expect(largestMove(fitInBox(polygon, size, anchor), centered)).toBeLessThan(
        EPSILON * Math.max(1, width, height),
      );
    },
  );

  test.prop({
    height: fc.integer({ max: 1000, min: 1 }),
    horizontal: ALIGNMENT,
    vertical: ALIGNMENT,
    width: fc.integer({ max: 1000, min: 1 }),
  })(
    "keeps the proportions: a square stays a square",
    ({ height, horizontal, vertical, width }) => {
      const box = boundingBox(fitInBox(SQUARE, { height, width }, { horizontal, vertical }));

      expect(box.maxX - box.minX).toBeCloseTo(box.maxY - box.minY, 9);
    },
  );
});
