import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { roundedContour } from "../corner-radius";

import { boundingBox } from "./bounding-box";
import { contourBoundingBox } from "./contour-bounding-box";

import type { Corner } from "../contour";

/**
 * Corners of a 100 × 100 square turned about (50, 50), all with one radius.
 *
 * @param angle - angle in degrees, clockwise on screen
 * @param radius - corner radius of each vertex
 * @returns the four corners, top-left first before turning
 */
function turnedSquare(angle: number, radius: number): readonly Corner[] {
  const radians = (angle * Math.PI) / 180;
  const cos = Math.cos(radians);
  const sin = Math.sin(radians);

  return [
    { x: -50, y: -50 },
    { x: 50, y: -50 },
    { x: 50, y: 50 },
    { x: -50, y: 50 },
  ].map((offset) => ({
    point: { x: 50 + offset.x * cos - offset.y * sin, y: 50 + offset.x * sin + offset.y * cos },
    radius,
  }));
}

describe("contourBoundingBox (DERIV-arc-bounding-box)", () => {
  it("[F04.AC3] is the box of the corners for a sharp square turned by 45°: 141.42136 wide", () => {
    // Corners at distance 50√2 from (50, 50) on the axes: 50 ± 70.71068.
    const box = contourBoundingBox(roundedContour(turnedSquare(45, 0)));

    expect(box.minX).toBeCloseTo(50 - 50 * Math.SQRT2, 9);
    expect(box.maxX).toBeCloseTo(50 + 50 * Math.SQRT2, 9);
    expect(box.minY).toBeCloseTo(50 - 50 * Math.SQRT2, 9);
    expect(box.maxY - box.minY).toBeCloseTo(100 * Math.SQRT2, 9);
  });

  it("[F04.AC3] counts the arcs' extreme points: radius 10 at 45° spans −16.56854 to 116.56854", () => {
    // Arc centers at distance 40√2 from (50, 50) on the axes, radius 10: 50 ± (40√2 + 10).
    // The tangent points only reach 50 ± 45√2 = 113.63961, inside.
    const box = contourBoundingBox(roundedContour(turnedSquare(45, 10)));
    const reach = 40 * Math.SQRT2 + 10;

    expect(box.minX).toBeCloseTo(50 - reach, 9);
    expect(box.maxX).toBeCloseTo(50 + reach, 9);
    expect(box.minY).toBeCloseTo(50 - reach, 9);
    expect(box.maxY).toBeCloseTo(50 + reach, 9);
  });

  it("[F04.AC3] is the square itself for a straight square with radius 10: the arc ends are on the edges", () => {
    const box = contourBoundingBox(roundedContour(turnedSquare(0, 10)));

    // The arc centers are computed (filletArc): an axis point may be off by an ulp, far below
    // the 5 decimals written (Q10).
    expect(box.minX).toBeCloseTo(0, 12);
    expect(box.maxX).toBeCloseTo(100, 12);
    expect(box.minY).toBeCloseTo(0, 12);
    expect(box.maxY).toBeCloseTo(100, 12);
  });

  test.prop({ angle: fc.integer({ max: 359, min: 0 }) })(
    "[F04.AC3] is the same square for a circle (a 100 square with radius 50) turned by any angle",
    ({ angle }) => {
      const box = contourBoundingBox(roundedContour(turnedSquare(angle, 50)));

      expect(box.minX).toBeCloseTo(0, 9);
      expect(box.maxX).toBeCloseTo(100, 9);
      expect(box.minY).toBeCloseTo(0, 9);
      expect(box.maxY).toBeCloseTo(100, 9);
    },
  );

  test.prop({ angle: fc.integer({ max: 359, min: 0 }), radius: fc.integer({ max: 60, min: 0 }) })(
    "contains every piece end and lies within the sharp corners' box",
    ({ angle, radius }) => {
      const corners = turnedSquare(angle, radius);
      const pieces = roundedContour(corners);
      const box = contourBoundingBox(pieces);
      const sharp = boundingBox(corners.map((corner) => corner.point));

      for (const piece of pieces) {
        expect(piece.end.x).toBeGreaterThanOrEqual(box.minX - 1e-9);
        expect(piece.end.x).toBeLessThanOrEqual(box.maxX + 1e-9);
        expect(piece.end.y).toBeGreaterThanOrEqual(box.minY - 1e-9);
        expect(piece.end.y).toBeLessThanOrEqual(box.maxY + 1e-9);
      }

      expect(box.minX).toBeGreaterThanOrEqual(sharp.minX - 1e-9);
      expect(box.minY).toBeGreaterThanOrEqual(sharp.minY - 1e-9);
      expect(box.maxX).toBeLessThanOrEqual(sharp.maxX + 1e-9);
      expect(box.maxY).toBeLessThanOrEqual(sharp.maxY + 1e-9);
    },
  );

  it("counts an arc without length by its ends only", () => {
    // A corner on a straight line keeps no fillet: center = start = end, radius 10.
    expect(
      contourBoundingBox([
        {
          center: { x: 5, y: 5 },
          end: { x: 5, y: 5 },
          kind: "arc",
          positive: true,
          radius: 10,
          start: { x: 5, y: 5 },
        },
      ]),
    ).toEqual({ maxX: 5, maxY: 5, minX: 5, minY: 5 });
  });
});
