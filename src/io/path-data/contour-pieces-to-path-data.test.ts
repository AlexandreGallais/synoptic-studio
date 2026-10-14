import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { rectangleCorners } from "../../../tests";
import { roundedContour } from "../../geometry";

import { contourPiecesToPathData } from "./contour-pieces-to-path-data";

describe("contourPiecesToPathData (REF-SVG2-PATHS §9.3.3–9.3.8)", () => {
  it("[F01.AC2] [F11.AC6] [F13.AC3] writes the 100 × 50 rectangle with radius 10, as before F11 (EN-007 criterion)", () => {
    const pieces = roundedContour(rectangleCorners(100, 50, [10, 10, 10, 10]));

    expect(contourPiecesToPathData(pieces)).toBe(
      "M10 0 L90 0 A10 10 0 0 1 100 10 L100 40 A10 10 0 0 1 90 50 L10 50 A10 10 0 0 1 0 40 L0 10 A10 10 0 0 1 10 0 Z",
    );
  });

  it("[F01.AC3] writes the circle of a square of 100 with radius 50: four arcs, no line", () => {
    const pieces = roundedContour(rectangleCorners(100, 100, [50, 50, 50, 50]));

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
    const pieces = roundedContour(rectangleCorners(0, 50, [0, 0, 0, 0]));

    expect(contourPiecesToPathData(pieces)).toBe("M0 0 L0 50 Z");
  });

  it("writes derived coordinates with fixed precision (EN-001)", () => {
    // Equilateral triangle of side 100: apex height 100 × √3 / 2 = 86.602540378… → 86.60254.
    const height = (100 * Math.sqrt(3)) / 2;
    const pieces = roundedContour([
      { point: { x: 50, y: 0 }, radius: 0 },
      { point: { x: 100, y: height }, radius: 0 },
      { point: { x: 0, y: height }, radius: 0 },
    ]);

    expect(contourPiecesToPathData(pieces)).toBe("M50 0 L100 86.60254 L0 86.60254 Z");
  });

  it("writes a concave fillet with sweep flag 0: the inner corner of an L shape", () => {
    // V = (50, 50) turns counterclockwise on screen (τ = −π/2): the arc runs from (60, 50) to (50, 60).
    const pieces = roundedContour(
      [
        { x: 0, y: 0 },
        { x: 100, y: 0 },
        { x: 100, y: 50 },
        { x: 50, y: 50 },
        { x: 50, y: 100 },
        { x: 0, y: 100 },
      ].map((point) => ({ point, radius: point.x === 50 && point.y === 50 ? 10 : 0 })),
    );

    expect(contourPiecesToPathData(pieces)).toBe(
      "M0 0 L100 0 L100 50 L60 50 A10 10 0 0 0 50 60 L50 100 L0 100 Z",
    );
  });

  it("gives an empty d for no piece", () => {
    expect(contourPiecesToPathData([])).toBe("");
  });

  test.prop({
    height: fc.integer({ max: 1000, min: 1 }),
    width: fc.integer({ max: 1000, min: 1 }),
  })("writes exactly the sharp path data of EN-002 when every radius is 0", ({ height, width }) => {
    const pieces = roundedContour(rectangleCorners(width, height, [0, 0, 0, 0]));

    // Three explicit edges, the fourth drawn by Z (SVG 2 §9.3.4).
    expect(contourPiecesToPathData(pieces)).toBe(
      `M0 0 L${String(width)} 0 L${String(width)} ${String(height)} L0 ${String(height)} Z`,
    );
  });

  it("[F12.AC3] writes a cubic side as C, and a curved closing side before Z", () => {
    // Triangle (0, 0) → (100, 0) curved through (30, −15) → (100, 100) → back to (0, 0) curved.
    expect(
      contourPiecesToPathData([
        {
          end: { x: 100, y: 0 },
          first: { x: 30, y: -15 },
          kind: "cubic",
          second: { x: 100, y: 0 },
          start: { x: 0, y: 0 },
        },
        { end: { x: 100, y: 100 }, kind: "segment", start: { x: 100, y: 0 } },
        {
          end: { x: 0, y: 0 },
          first: { x: 80, y: 120 },
          kind: "cubic",
          second: { x: -20, y: 30 },
          start: { x: 100, y: 100 },
        },
      ]),
    ).toBe("M0 0 C30 -15 100 0 100 0 L100 100 C80 120 -20 30 0 0 Z");
  });
});
