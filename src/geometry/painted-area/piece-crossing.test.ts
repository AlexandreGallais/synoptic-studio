import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { straightCubic } from "../curve";

import { pieceCrossing } from "./piece-crossing";

import type { ContourPiece } from "../contour";

/** A piece and its count from (50, 50). */
type Row = {
  /** The signed count expected. */
  readonly expected: number;
  /** A segment, an arc or a cubic side. */
  readonly piece: ContourPiece;
};

describe("pieceCrossing (DERIV-painted-area steps 2 to 4)", () => {
  it.each<Row>([
    // The right side of a box 100 × 100, y growing, right of (50, 50): +1.
    { expected: 1, piece: { end: { x: 100, y: 100 }, kind: "segment", start: { x: 100, y: 0 } } },
    // A fillet from (40, 100) to (60, 80) about (40, 80), radius 20, turning back: its chord
    // spans y from 80 to 100 only, and (50, 50) is 31.6 from its centre: 0.
    {
      expected: 0,
      piece: {
        center: { x: 40, y: 80 },
        end: { x: 60, y: 80 },
        kind: "arc",
        positive: false,
        radius: 20,
        start: { x: 40, y: 100 },
      },
    },
    // A fillet from (60, 60) up to (80, 40) about (80, 60), radius 20: its chord crosses y = 50
    // at x = 70, right of 50, y falling: −1; (50, 50) is 31.6 from the centre, outside.
    {
      expected: -1,
      piece: {
        center: { x: 80, y: 60 },
        end: { x: 80, y: 40 },
        kind: "arc",
        positive: true,
        radius: 20,
        start: { x: 60, y: 60 },
      },
    },
    // An arch from (0, 70) to (100, 70), 30 high: y = 70 − 120 t (1 − t) is 50 at
    // t = (1 ∓ √⅓) / 2, x ≈ 11.51 (y falling) and 88.49 (y growing): only the second, +1.
    {
      expected: 1,
      piece: {
        end: { x: 100, y: 70 },
        first: { x: 0, y: 30 },
        kind: "cubic",
        second: { x: 100, y: 30 },
        start: { x: 0, y: 70 },
      },
    },
  ])("counts $expected for a $piece.kind", ({ expected, piece }) => {
    expect(pieceCrossing(piece, { x: 50, y: 50 })).toBe(expected);
  });
});

describe("pieceCrossing (properties)", () => {
  /** A point on integers. */
  const integerPoint = fc.record({
    x: fc.integer({ max: 100, min: -100 }),
    y: fc.integer({ max: 100, min: -100 }),
  });

  test.prop({ end: integerPoint, point: integerPoint, start: integerPoint })(
    "counts a straight cubic side as the segment it draws",
    ({ end, point, start }) => {
      // Away from the line, where the crossing's rounding could matter.
      const side =
        (end.x - start.x) * (point.y - start.y) - (end.y - start.y) * (point.x - start.x);

      fc.pre(Math.abs(side) > 1);

      expect(pieceCrossing(straightCubic(start, end), point)).toBe(
        pieceCrossing({ end, kind: "segment", start }, point),
      );
    },
  );
});
