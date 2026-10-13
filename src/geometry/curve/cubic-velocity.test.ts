import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";
import { cubicPoint } from "../placement";

import { cubicVelocity } from "./cubic-velocity";

import type { Cubic } from "../contour";

/** A cubic with integer points. */
const CUBIC = fc
  .tuple(integerPoint(100), integerPoint(100), integerPoint(100), integerPoint(100))
  .map(([start, first, second, end]): Cubic => ({ end, first, kind: "cubic", second, start }));

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
});
