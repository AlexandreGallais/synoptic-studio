import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { roundedContour } from "../geometry";

import { contourPiecesToPathData } from "./contour-pieces-to-path-data";
import { contourToPathData } from "./contour-to-path-data";

import type { Corner } from "../geometry";
import type { Point } from "../math";

/**
 * Rectangle clockwise from the top-left vertex (Q11).
 *
 * @param width - width, >= 0
 * @param height - height, >= 0
 * @returns its four vertices
 */
function rectangle(width: number, height: number): Point[] {
  return [
    { x: 0, y: 0 },
    { x: width, y: 0 },
    { x: width, y: height },
    { x: 0, y: height },
  ];
}

/**
 * Corners of a rectangle with one radius everywhere.
 *
 * @param width - width, >= 0
 * @param height - height, >= 0
 * @param radius - radius requested at every corner
 * @returns its four corners
 */
function corners(width: number, height: number, radius: number): Corner[] {
  return rectangle(width, height).map((point) => ({ point, radius }));
}

describe("contourPiecesToPathData (REF-SVG2-PATHS §9.3.3–9.3.8)", () => {
  it("[F01.AC2] writes the 100 × 50 rectangle with radius 10 (EN-007 criterion)", () => {
    const pieces = roundedContour(corners(100, 50, 10));

    expect(contourPiecesToPathData(pieces)).toBe(
      "M10 0 L90 0 A10 10 0 0 1 100 10 L100 40 A10 10 0 0 1 90 50 L10 50 A10 10 0 0 1 0 40 L0 10 A10 10 0 0 1 10 0 Z",
    );
  });

  it("[F01.AC3] writes the circle of a square of 100 with radius 50: four arcs, no line", () => {
    const pieces = roundedContour(corners(100, 100, 50));

    expect(contourPiecesToPathData(pieces)).toBe(
      "M50 0 A50 50 0 0 1 100 50 A50 50 0 0 1 50 100 A50 50 0 0 1 0 50 A50 50 0 0 1 50 0 Z",
    );
  });

  it("leaves the final straight edge to closepath", () => {
    // Sharp corner 0, rounded corners elsewhere: the last piece is the left edge, drawn by Z.
    const pieces = roundedContour([
      { point: { x: 0, y: 0 }, radius: 0 },
      { point: { x: 100, y: 0 }, radius: 10 },
      { point: { x: 100, y: 50 }, radius: 10 },
      { point: { x: 0, y: 50 }, radius: 10 },
    ]);

    expect(contourPiecesToPathData(pieces)).toBe(
      "M0 0 L90 0 A10 10 0 0 1 100 10 L100 40 A10 10 0 0 1 90 50 L10 50 A10 10 0 0 1 0 40 Z",
    );
  });

  it("drops the zero-length edges of a rectangle of size 0 (Q15)", () => {
    // 0 × 50: two segments remain, (0, 0) → (0, 50) and back, the second left to Z.
    const pieces = roundedContour(corners(0, 50, 0));

    expect(contourPiecesToPathData(pieces)).toBe("M0 0 L0 50 Z");
  });

  it("gives an empty d for no piece", () => {
    expect(contourPiecesToPathData([])).toBe("");
  });

  test.prop({
    height: fc.integer({ max: 1000, min: 1 }),
    width: fc.integer({ max: 1000, min: 1 }),
  })("writes exactly the sharp path data of EN-002 when every radius is 0", ({ height, width }) => {
    const pieces = roundedContour(corners(width, height, 0));

    expect(contourPiecesToPathData(pieces)).toBe(contourToPathData(rectangle(width, height)));
  });
});
