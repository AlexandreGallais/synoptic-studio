import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerTriangle, rectangleCorners } from "../../../tests";

import { cornerSetbacks } from "./corner-setbacks";
import { cornerTriples } from "./corner-triples";
import { turningAngles } from "./turning-angles";

import type { Corner } from "../corner";

/**
 * Setbacks of the corners of a contour, their turning angles computed from their points.
 *
 * @param corners - vertices with their requested radii, in drawing order
 * @returns one setback per corner
 */
function setbacksOf(corners: readonly Corner[]): readonly number[] {
  const triples = cornerTriples(corners.map((corner) => corner.point));

  return cornerSetbacks(corners, turningAngles(triples));
}

/** Decimals compared: tangents of doubles, correct to their last bits. */
const DECIMALS = 9;

describe("cornerSetbacks (DERIV-local-radius-clamp step 1)", () => {
  it("equals the radius at the right corners of a square", () => {
    // s = r · tan(π/4) = r.
    const setbacks = setbacksOf(rectangleCorners(100, 100, [1000, 0, 50, 10]));

    expect(setbacks[0]).toBeCloseTo(1000, DECIMALS);
    expect(setbacks[1]).toBe(0);
    expect(setbacks[2]).toBeCloseTo(50, DECIMALS);
    expect(setbacks[3]).toBeCloseTo(10, DECIMALS);
  });

  it("is 0 at an aligned vertex, whatever its radius", () => {
    // (50, 0) lies on the top edge: τ = 0, s = 30 · tan(0) = 0.
    const corners = [
      { point: { x: 0, y: 0 }, radius: 0 },
      { point: { x: 50, y: 0 }, radius: 30 },
      { point: { x: 100, y: 0 }, radius: 0 },
      { point: { x: 100, y: 50 }, radius: 0 },
      { point: { x: 0, y: 50 }, radius: 0 },
    ];

    expect(setbacksOf(corners)[1]).toBe(0);
  });

  it("is 0 everywhere on a rectangle of size 0, without NaN (Q15)", () => {
    // Zero-length edges: τ = atan2(0, 0) = 0 at every vertex.
    const corners = [
      { point: { x: 0, y: 0 }, radius: 10 },
      { point: { x: 0, y: 0 }, radius: 10 },
      { point: { x: 0, y: 50 }, radius: 10 },
      { point: { x: 0, y: 50 }, radius: 10 },
    ];

    expect(setbacksOf(corners)).toEqual([0, 0, 0, 0]);
  });
});

/** Arbitrary requested radius. */
const RADIUS = fc.integer({ max: 5000, min: 0 });

/** Triangles of positive area, a radius at each corner. */
const TRIANGLE = integerTriangle(1000).chain((points) =>
  fc
    .tuple(RADIUS, RADIUS, RADIUS)
    .map((radii) => points.map((point, index) => ({ point, radius: radii[index] ?? 0 }))),
);

describe("cornerSetbacks (properties)", () => {
  test.prop({ corners: TRIANGLE })(
    "gives the same setback at each corner when the contour is drawn the other way",
    ({ corners }) => {
      // Reversing the order flips the sign of each turn, not its size: s = r · tan(|τ| / 2).
      const reversed = setbacksOf(corners.toReversed());

      for (const [index, setback] of setbacksOf(corners).toReversed().entries()) {
        expect(reversed[index]).toBeCloseTo(setback, 6);
      }
    },
  );

  test.prop({
    height: fc.integer({ max: 5000, min: 1 }),
    radii: fc.tuple(RADIUS, RADIUS, RADIUS, RADIUS),
    width: fc.integer({ max: 5000, min: 1 }),
    x: fc.integer({ max: 5000, min: -5000 }),
    y: fc.integer({ max: 5000, min: -5000 }),
  })(
    "equals the radius at every right corner of any rectangle, wherever it is",
    ({ height, radii, width, x, y }) => {
      // Every corner of a rectangle turns π/2: s = r · tan(π/4) = r.
      const points = [
        { x, y },
        { x: x + width, y },
        { x: x + width, y: y + height },
        { x, y: y + height },
      ];
      const setbacks = setbacksOf(
        points.map((point, index) => ({ point, radius: radii[index] ?? 0 })),
      );

      for (const [index, setback] of setbacks.entries()) {
        expect(setback).toBeCloseTo(radii[index] ?? 0, 9);
      }
    },
  );
});
