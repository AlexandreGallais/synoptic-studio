import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerTriangle, rectangleCorners } from "../../../tests";
import { EPSILON } from "../../math";

import { effectiveRadii } from "./effective-radii";
import { roundedContour } from "./rounded-contour";

import type { Point } from "../../math";
import type { Corner } from "../corner";

/** Decimals compared, from `EPSILON = 1e-9` (Q10). */
const DECIMALS = 9;

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
    expect(outline(rectangleCorners(100, 50, [10, 10, 10, 10]))).toEqual([
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
    expect(outline(rectangleCorners(100, 100, [50, 50, 50, 50]))).toEqual([
      "arc 50,0 → 100,50",
      "arc 100,50 → 50,100",
      "arc 50,100 → 0,50",
      "arc 0,50 → 50,0",
    ]);
  });

  it("gives the sharp contour with radius 0", () => {
    expect(outline(rectangleCorners(100, 50, [0, 0, 0, 0]))).toEqual([
      "segment 0,0 → 100,0",
      "segment 100,0 → 100,50",
      "segment 100,50 → 0,50",
      "segment 0,50 → 0,0",
    ]);
  });

  it("keeps the two sides of a rectangle of size 0, without NaN (Q15)", () => {
    // 0 × 50: the top and bottom edges have no length and no corner can be rounded.
    expect(outline(rectangleCorners(0, 50, [10, 10, 10, 10]))).toEqual([
      "segment 0,0 → 0,50",
      "segment 0,50 → 0,0",
    ]);
  });

  it("rounds a concave corner the other way: the inner corner of an L shape", () => {
    // V = (50, 50) between (100, 50) and (50, 100): τ = −π/2 (counterclockwise on screen),
    // s = 10 · tan(π/4) = 10, T_in = (60, 50), T_out = (50, 60), C = T_in − 10 · (0, −1) = (60, 60).
    const arcs = roundedContour(
      [
        { x: 0, y: 0 },
        { x: 100, y: 0 },
        { x: 100, y: 50 },
        { x: 50, y: 50 },
        { x: 50, y: 100 },
        { x: 0, y: 100 },
      ].map((point) => ({ point, radius: point.x === 50 && point.y === 50 ? 10 : 0 })),
    ).filter((piece) => piece.kind === "arc");

    expect(arcs).toEqual([
      {
        center: { x: 60, y: 60 },
        end: { x: 50, y: 60 },
        kind: "arc",
        positive: false,
        radius: 10,
        start: { x: 60, y: 50 },
      },
    ]);
  });

  it("clamps each corner by its own turn: radius 10 on a 3-4-5 triangle gives its incircle, radius 1", () => {
    // Setbacks 10 · tan(|τ|/2) = 10, 30, 20; every edge factor 0.1 (4/40, 5/50, 3/30): r′ = 1, the
    // inradius (3 + 4 − 5) / 2.
    const arcs = roundedContour(
      [
        { x: 0, y: 0 },
        { x: 4, y: 0 },
        { x: 0, y: 3 },
      ].map((point) => ({ point, radius: 10 })),
    ).filter((piece) => piece.kind === "arc");

    expect(arcs).toHaveLength(3);

    for (const arc of arcs) {
      expect(arc.radius).toBeCloseTo(1, DECIMALS);
    }
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
      const pieces = roundedContour(
        rectangleCorners(width, height, [radius, radius, radius, radius]),
      );

      for (const [index, piece] of pieces.entries()) {
        const next = pieces[(index + 1) % pieces.length] ?? piece;

        expect(Math.hypot(next.start.x - piece.end.x, next.start.y - piece.end.y)).toBeLessThan(
          EPSILON * 1000,
        );
      }
    },
  );

  test.prop({
    radii: fc.tuple(
      fc.integer({ max: 2000, min: 1 }),
      fc.integer({ max: 2000, min: 1 }),
      fc.integer({ max: 2000, min: 1 }),
    ),
    triangle: integerTriangle(1000),
  })(
    "draws each corner's arc with the effective radius of that corner, in order",
    ({ radii, triangle }) => {
      // Every corner of a triangle turns: one arc per corner, the first one closing the contour.
      const corners = triangle.map((point, index) => ({ point, radius: radii[index] ?? 0 }));
      const effective = effectiveRadii(corners);
      const arcs = roundedContour(corners).filter((piece) => piece.kind === "arc");
      const drawn = arcs.map((arc) => arc.radius);

      // The contour starts after the first corner's arc: arcs of corners 1, 2, then 0.
      expect(drawn).toEqual([effective[1], effective[2], effective[0]]);
    },
  );
});
