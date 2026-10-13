import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";

import { stretchFactors } from "./stretch-factors";
import { stretchToSize } from "./stretch-to-size";

/** The unit square, a free shape's stored rectangle (ADR-0032). */
const UNIT_SQUARE = [
  { x: 0, y: 0 },
  { x: 1, y: 0 },
  { x: 1, y: 1 },
  { x: 0, y: 1 },
];

describe("stretchFactors (DERIV-free-shape-stretch)", () => {
  it("divides the size asked by the span of the points", () => {
    // The unit square drawn 120 × 80: factors 120 and 80.
    expect(stretchFactors(UNIT_SQUARE, { height: 80, width: 120 })).toEqual({ x: 120, y: 80 });
  });

  it("gives 1 on an axis already at its size, even of span 0", () => {
    // A flat shape of span 0 × 50 drawn 0 × 100: no division on x.
    const flat = [
      { x: 5, y: 0 },
      { x: 5, y: 50 },
    ];

    expect(stretchFactors(flat, { height: 100, width: 0 })).toEqual({ x: 1, y: 2 });
  });

  test.prop({ points: fc.array(integerPoint(1e3), { minLength: 2 }), size: integerPoint(1e3) })(
    "moves each point away from the box's top-left by its offset times the factors",
    ({ points, size }) => {
      const width = Math.abs(size.x);
      const height = Math.abs(size.y);
      const xs = points.map((point) => point.x);
      const ys = points.map((point) => point.y);

      fc.pre(Math.max(...xs) > Math.min(...xs) && Math.max(...ys) > Math.min(...ys));

      const factors = stretchFactors(points, { height, width });
      const stretched = stretchToSize(points, { height, width });

      for (const [index, point] of stretched.entries()) {
        const source = points[index] ?? point;

        expect(point.x - Math.min(...xs)).toBeCloseTo(factors.x * (source.x - Math.min(...xs)), 6);
        expect(point.y - Math.min(...ys)).toBeCloseTo(factors.y * (source.y - Math.min(...ys)), 6);
      }
    },
  );
});
