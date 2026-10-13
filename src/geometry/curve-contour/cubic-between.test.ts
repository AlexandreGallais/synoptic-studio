import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";
import { cubicPoint } from "../placement";

import { cubicBetween } from "./cubic-between";

import type { Cubic } from "../contour";

/** A cubic with integer points. */
const CUBIC = fc
  .tuple(integerPoint(100), integerPoint(100), integerPoint(100), integerPoint(100))
  .map(([start, first, second, end]): Cubic => ({ end, first, kind: "cubic", second, start }));

describe("cubicBetween (DERIV-fillet-beside-curve step 9)", () => {
  it("keeps the cubic itself from 0 to 1, and cuts a straight one at its shares", () => {
    const straight: Cubic = {
      end: { x: 90, y: 0 },
      first: { x: 30, y: 0 },
      kind: "cubic",
      second: { x: 60, y: 0 },
      start: { x: 0, y: 0 },
    };

    expect(cubicBetween(straight, 0, 1)).toBe(straight);
    expect(cubicBetween(straight, 0.2, 0.6).start.x).toBeCloseTo(18, 12);
    expect(cubicBetween(straight, 0.2, 0.6).end.x).toBeCloseTo(54, 12);
  });

  test.prop({
    cubic: CUBIC,
    from: fc.double({ max: 0.4, min: 0, noNaN: true }),
    s: fc.double({ max: 1, min: 0, noNaN: true }),
    to: fc.double({ max: 1, min: 0.6, noNaN: true }),
  })("lies on the cubic, from one parameter to the other", ({ cubic, from, s, to }) => {
    const piece = cubicPoint(cubicBetween(cubic, from, to), s);
    const whole = cubicPoint(cubic, from + s * (to - from));

    expect(piece.x).toBeCloseTo(whole.x, 6);
    expect(piece.y).toBeCloseTo(whole.y, 6);
  });
});
