import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { boundingBox, effectiveRadii, roundedContour } from "../geometry";

import { roundShapeCorners } from "./round-shape-corners";
import { turnedCorners } from "./turned-corners";

import type { RoundShape } from "./round-shape";
import type { ContourPiece } from "../geometry";

/**
 * A round shape straight at (0, 0).
 *
 * @param width - width of its box
 * @param height - height of its box
 * @returns the round shape
 */
function round(width: number, height: number): RoundShape {
  return { angle: 0, height, kind: "round", width, x: 0, y: 0 };
}

/**
 * A number rounded to 6 decimals, a negative zero made positive.
 *
 * @param value - number to round
 * @returns the rounded number
 */
function round6(value: number): number {
  return Math.round(value * 1e6) / 1e6 + 0;
}

/**
 * Kind, end and radius of each piece of a drawn contour, rounded to 6 decimals.
 *
 * @param pieces - segments and arcs
 * @returns one line per piece
 */
function summary(pieces: readonly ContourPiece[]): readonly string[] {
  return pieces.map((piece) => {
    const radius = piece.kind === "arc" ? ` r${String(round6(piece.radius))}` : "";

    return `${piece.kind} ${String(round6(piece.end.x))} ${String(round6(piece.end.y))}${radius}`;
  });
}

describe("roundShapeCorners (DERIV-round-shape step 1)", () => {
  it("[F10.AC1] draws the circle of diameter 100 in a 100 × 100 box: four quarter arcs of radius 50", () => {
    // Each corner rounded by 50: the arcs meet in the middle of every side (DERIV-round-shape).
    const pieces = roundedContour(roundShapeCorners(round(100, 100)));

    expect(summary(pieces)).toEqual([
      "arc 100 50 r50",
      "arc 50 100 r50",
      "arc 0 50 r50",
      "arc 50 0 r50",
    ]);
  });

  it("[F10.AC2] draws a capsule in 120 × 80: half circles of radius 40, straight sides from 40 to 80", () => {
    const pieces = roundedContour(roundShapeCorners(round(120, 80)));

    // The straight parts of the short sides, 1.4e-14 long in floating point, are dropped.
    expect(summary(pieces)).toEqual([
      "segment 80 0",
      "arc 120 40 r40",
      "arc 80 80 r40",
      "segment 40 80",
      "arc 0 40 r40",
      "arc 40 0 r40",
    ]);
  });

  it("[F10.AC1] draws a line for a round shape of 0 × 50, as a rectangle of width 0", () => {
    const pieces = roundedContour(roundShapeCorners(round(0, 50)));

    expect(summary(pieces)).toEqual(["segment 0 50", "segment 0 0"]);
  });

  it("[F10.AC2] reduces no radius: every corner keeps half the smaller side", () => {
    const radii = effectiveRadii(roundShapeCorners(round(80, 120)));

    for (const radius of radii) {
      expect(radius).toBeCloseTo(40, 9);
    }
  });

  it("[F10.AC2] turns and moves as every shape: 120 × 80 at (10, 20), 90°", () => {
    // Center (70, 60); point 1 (10, 20) ↦ (70 + 40, 60 − 60) = (110, 0).
    const shape = { ...round(120, 80), angle: 90, x: 10, y: 20 };

    expect(turnedCorners(roundShapeCorners(shape), shape)[0]?.point).toEqual({ x: 110, y: 0 });
  });

  test.prop({
    height: fc.integer({ max: 1000, min: 1 }),
    width: fc.integer({ max: 1000, min: 1 }),
  })(
    "[F10.AC1] fills its box, its corners rounded by half the smaller side",
    ({ height, width }) => {
      const corners = roundShapeCorners(round(width, height));
      const box = boundingBox(corners.map((corner) => corner.point));

      expect([box.minX, box.minY, box.maxX, box.maxY]).toEqual([0, 0, width, height]);

      expect(corners.map((corner) => corner.radius)).toEqual(
        Array.from({ length: 4 }, () => Math.min(width, height) / 2),
      );
    },
  );
});
