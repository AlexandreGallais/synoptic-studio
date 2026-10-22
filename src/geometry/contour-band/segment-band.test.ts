import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { segmentBand } from "./segment-band";

import type { Point } from "../../math";
import type { Segment } from "../contour";

/**
 * Twice the signed area of a polygon, the shoelace sum: positive when it turns in the positive
 * direction of angles, clockwise on screen (y down).
 *
 * @param points - its vertices, in order
 * @returns Σ (xᵢ yᵢ₊₁ − xᵢ₊₁ yᵢ)
 */
function shoelace(points: readonly Point[]): number {
  return points.reduce((sum, point, index) => {
    const next = points[(index + 1) % points.length] ?? point;

    return sum + point.x * next.y - next.x * point.y;
  }, 0);
}

/**
 * A segment from one point to another.
 *
 * @param start - where it starts
 * @param end - where it ends
 * @returns the segment from start to end
 */
function segment(start: Point, end: Point): Segment {
  return { end, kind: "segment", start };
}

describe("segmentBand", () => {
  it("gives the rectangle 10 × 2 about a segment along x, from A − n", () => {
    // d = (10, 0), unit (1, 0), n = 1 · (−0, 1): A − n = (0, −1), B − n = (10, −1), B + n = (10, 1),
    // A + n = (0, 1).
    expect(segmentBand(segment({ x: 0, y: 0 }, { x: 10, y: 0 }), 1)).toEqual([
      segment({ x: 0, y: -1 }, { x: 10, y: -1 }),
      segment({ x: 10, y: -1 }, { x: 10, y: 1 }),
      segment({ x: 10, y: 1 }, { x: 0, y: 1 }),
      segment({ x: 0, y: 1 }, { x: 0, y: -1 }),
    ]);
  });

  it("gives the rectangle 4 × 10 about a segment down, turning clockwise on screen", () => {
    // d = (0, 10), unit (0, 1), n = 2 · (−1, 0) = (−2, 0): A − n = (2, 0), B − n = (2, 10),
    // B + n = (−2, 10), A + n = (−2, 0); shoelace 20 + 40 + 20 + 0 = 80 = 2 · 4 · 10, positive.
    const sides = segmentBand(segment({ x: 0, y: 0 }, { x: 0, y: 10 }), 2);

    expect(sides.map((side) => side.start)).toEqual([
      { x: 2, y: 0 },
      { x: 2, y: 10 },
      { x: -2, y: 10 },
      { x: -2, y: 0 },
    ]);

    expect(shoelace(sides.map((side) => side.start))).toBe(80);
  });

  it("gives none for a segment of length 0, which has no direction", () => {
    expect(segmentBand(segment({ x: 5, y: 5 }, { x: 5, y: 5 }), 1)).toEqual([]);
  });
});

describe("segmentBand (properties)", () => {
  /** A point on integers of a drawing. */
  const point = fc.record({
    x: fc.integer({ max: 1000, min: -1000 }),
    y: fc.integer({ max: 1000, min: -1000 }),
  });

  test.prop({ end: point, halfWidth: fc.integer({ max: 10, min: 1 }), start: point })(
    "is a closed rectangle of area 2h · |AB|, turning clockwise on screen, its sides h from AB",
    ({ end, halfWidth, start }) => {
      fc.pre(start.x !== end.x || start.y !== end.y);

      const sides = segmentBand(segment(start, end), halfWidth);
      const corners = sides.map((side) => side.start);
      const length = Math.hypot(end.x - start.x, end.y - start.y);

      expect(sides).toHaveLength(4);
      expect(sides.map((side) => side.end)).toEqual([...corners.slice(1), corners[0]]);
      expect(shoelace(corners) / 2).toBeCloseTo(2 * halfWidth * length, 6);

      for (const corner of corners) {
        // Distance from the corner to the line AB: |d ⊥· (P − A)| / |d|.
        const across =
          Math.abs(
            (end.x - start.x) * (corner.y - start.y) - (end.y - start.y) * (corner.x - start.x),
          ) / length;

        expect(across).toBeCloseTo(halfWidth, 6);
      }
    },
  );
});
