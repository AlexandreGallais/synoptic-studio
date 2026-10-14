import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";
import { cubicPoint, derivativeCoefficients } from "../placement";

import { cubicVelocity } from "./cubic-velocity";

import type { Cubic } from "../contour";

/** A cubic with integer points. */
const CUBIC = fc
  .tuple(integerPoint(100), integerPoint(100), integerPoint(100), integerPoint(100))
  .map(([start, first, second, end]): Cubic => ({ end, first, kind: "cubic", second, start }));

/**
 * The derivative of a cubic's coordinate in its power form, a t² + b t + c.
 *
 * @param weights - the coordinate of its four control points
 * @param t - the parameter
 * @returns the derivative at t
 */
function powerForm(weights: readonly [number, number, number, number], t: number): number {
  const { a, b, c } = derivativeCoefficients(weights);

  return a * t * t + b * t + c;
}

describe("cubicVelocity (REF-WIKI-BEZIER)", () => {
  it("is three times the first handle at the start, and the last at the end", () => {
    // B′(0) = 3(P₁ − P₀) = (90, 0); B′(1) = 3(P₃ − P₂) = (0, 60).
    const cubic: Cubic = {
      end: { x: 50, y: 40 },
      first: { x: 30, y: 0 },
      kind: "cubic",
      second: { x: 50, y: 20 },
      start: { x: 0, y: 0 },
    };

    expect([cubicVelocity(cubic, 0), cubicVelocity(cubic, 1)]).toEqual([
      { x: 90, y: 0 },
      { x: 0, y: 60 },
    ]);
  });

  test.prop({ cubic: CUBIC, t: fc.double({ max: 0.9, min: 0.1, noNaN: true }) })(
    "is the slope of the curve: a central difference agrees",
    ({ cubic, t }) => {
      const step = 1e-6;
      const ahead = cubicPoint(cubic, t + step);
      const behind = cubicPoint(cubic, t - step);
      const velocity = cubicVelocity(cubic, t);

      expect(velocity.x).toBeCloseTo((ahead.x - behind.x) / (2 * step), 3);
      expect(velocity.y).toBeCloseTo((ahead.y - behind.y) / (2 * step), 3);
    },
  );

  test.prop({ cubic: CUBIC, t: fc.double({ max: 1, min: 0, noNaN: true }) })(
    "agrees with the derivative's power form used for the box (DERIV-cubic-sides step 2)",
    ({ cubic, t }) => {
      const xs = [cubic.start.x, cubic.first.x, cubic.second.x, cubic.end.x] as const;
      const ys = [cubic.start.y, cubic.first.y, cubic.second.y, cubic.end.y] as const;

      expect(cubicVelocity(cubic, t).x).toBeCloseTo(powerForm(xs, t), 6);
      expect(cubicVelocity(cubic, t).y).toBeCloseTo(powerForm(ys, t), 6);
    },
  );
});
