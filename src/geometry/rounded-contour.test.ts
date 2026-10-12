import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { EPSILON } from "../math";

import { roundedContour } from "./rounded-contour";

import type { Corner } from "./corner";
import type { Point } from "../math";

/** Decimals compared, from `EPSILON = 1e-9` (Q10). */
const DECIMALS = 9;

/**
 * Corners of a rectangle clockwise from the top-left vertex (Q11), one radius everywhere.
 *
 * @param width - width, >= 0
 * @param height - height, >= 0
 * @param radius - radius requested at every corner
 * @returns the four corners
 */
function rectangle(width: number, height: number, radius: number): Corner[] {
  return [
    { x: 0, y: 0 },
    { x: width, y: 0 },
    { x: width, y: height },
    { x: 0, y: height },
  ].map((point) => ({ point, radius }));
}

/**
 * Point written `x,y`, coordinates rounded to 9 decimals.
 *
 * @param point - point to write
 * @returns its rounded coordinates
 */
function text(point: Point): string {
  return `${String(Number(point.x.toFixed(DECIMALS)))},${String(Number(point.y.toFixed(DECIMALS)))}`;
}

/**
 * Pieces reduced to their kind and rounded end points, for exact comparison.
 *
 * @param corners - corners of the contour
 * @returns `kind start → end` per piece, coordinates rounded to 9 decimals
 */
function outline(corners: readonly Corner[]): string[] {
  return roundedContour(corners).map(
    (piece) => `${piece.kind} ${text(piece.start)} → ${text(piece.end)}`,
  );
}

describe("roundedContour (DERIV-fillet-arc, ADR-0005 stage 3)", () => {
  it("alternates segments and quarter arcs on a 100 × 50 rectangle with radius 10", () => {
    // Starts at T_out of the top-left vertex (DERIV-fillet-arc step 7), its arc closes the contour.
    expect(outline(rectangle(100, 50, 10))).toEqual([
      "segment 10,0 → 90,0",
      "arc 90,0 → 100,10",
      "segment 100,10 → 100,40",
      "arc 100,40 → 90,50",
      "segment 90,50 → 10,50",
      "arc 10,50 → 0,40",
      "segment 0,40 → 0,10",
      "arc 0,10 → 10,0",
    ]);
  });

  it("[F01.AC3] gives 4 quarter arcs and no segment on a square of 100 with radius 50: a circle", () => {
    expect(outline(rectangle(100, 100, 50))).toEqual([
      "arc 50,0 → 100,50",
      "arc 100,50 → 50,100",
      "arc 50,100 → 0,50",
      "arc 0,50 → 50,0",
    ]);
  });

  it("gives the sharp contour with radius 0", () => {
    expect(outline(rectangle(100, 50, 0))).toEqual([
      "segment 0,0 → 100,0",
      "segment 100,0 → 100,50",
      "segment 100,50 → 0,50",
      "segment 0,50 → 0,0",
    ]);
  });

  it("keeps the two sides of a rectangle of size 0, without NaN (Q15)", () => {
    // 0 × 50: the top and bottom edges have no length and no corner can be rounded.
    expect(outline(rectangle(0, 50, 10))).toEqual(["segment 0,0 → 0,50", "segment 0,50 → 0,0"]);
  });

  it("gives nothing for an empty contour", () => {
    expect(roundedContour([])).toEqual([]);
  });

  test.prop({
    height: fc.integer({ max: 1000, min: 0 }),
    radius: fc.integer({ max: 2000, min: 0 }),
    width: fc.integer({ max: 1000, min: 0 }),
  })(
    "is a closed chain: each piece starts where the previous one ends",
    ({ height, radius, width }) => {
      const pieces = roundedContour(rectangle(width, height, radius));

      for (const [index, piece] of pieces.entries()) {
        const next = pieces[(index + 1) % pieces.length] ?? piece;

        expect(Math.hypot(next.start.x - piece.end.x, next.start.y - piece.end.y)).toBeLessThan(
          EPSILON * 1000,
        );
      }
    },
  );
});
