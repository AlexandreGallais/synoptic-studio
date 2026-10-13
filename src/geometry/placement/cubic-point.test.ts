import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";

import { cubicPoint } from "./cubic-point";

import type { Cubic } from "../contour";

/** The arch of `DERIV-cubic-sides`: (0, 0), (0, −40), (100, −40), (100, 0). */
const ARCH: Cubic = {
  end: { x: 100, y: 0 },
  first: { x: 0, y: -40 },
  kind: "cubic",
  second: { x: 100, y: -40 },
  start: { x: 0, y: 0 },
};

/** A cubic with integer points. */
const CUBIC = fc
  .tuple(integerPoint(1e3), integerPoint(1e3), integerPoint(1e3), integerPoint(1e3))
  .map(([start, first, second, end]): Cubic => ({ end, first, kind: "cubic", second, start }));

describe("cubicPoint (REF-WIKI-BEZIER)", () => {
  it("starts at the start, ends at the end, and passes (50, −30) at its middle", () => {
    // B(½) = (P₀ + 3P₁ + 3P₂ + P₃) / 8: y = 6 · (−40) / 8 = −30, x = (300 + 100) / 8 = 50.
    expect(cubicPoint(ARCH, 0)).toEqual({ x: 0, y: 0 });
    expect(cubicPoint(ARCH, 1)).toEqual({ x: 100, y: 0 });
    expect(cubicPoint(ARCH, 0.5)).toEqual({ x: 50, y: -30 });
  });

  test.prop({ cubic: CUBIC, t: fc.double({ max: 1, min: 0, noNaN: true }) })(
    "stays in the box of its four points (convex hull, REF-WIKI-BEZIER)",
    ({ cubic, t }) => {
      const point = cubicPoint(cubic, t);
      const xs = [cubic.start.x, cubic.first.x, cubic.second.x, cubic.end.x];
      const ys = [cubic.start.y, cubic.first.y, cubic.second.y, cubic.end.y];

      expect(point.x).toBeGreaterThanOrEqual(Math.min(...xs) - 1e-9);
      expect(point.x).toBeLessThanOrEqual(Math.max(...xs) + 1e-9);
      expect(point.y).toBeGreaterThanOrEqual(Math.min(...ys) - 1e-9);
      expect(point.y).toBeLessThanOrEqual(Math.max(...ys) + 1e-9);
    },
  );
});
