import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";
import { cubicPoint } from "../placement";

import { reversedCubic } from "./reversed-cubic";

import type { Cubic } from "../contour";

/** A cubic with integer points. */
const CUBIC = fc
  .tuple(integerPoint(100), integerPoint(100), integerPoint(100), integerPoint(100))
  .map(([start, first, second, end]): Cubic => ({ end, first, kind: "cubic", second, start }));

describe("reversedCubic (DERIV-fillet-beside-curve step 0)", () => {
  it("swaps the ends and the control points", () => {
    const cubic: Cubic = {
      end: { x: 3, y: 3 },
      first: { x: 1, y: 1 },
      kind: "cubic",
      second: { x: 2, y: 2 },
      start: { x: 0, y: 0 },
    };

    expect(reversedCubic(cubic)).toEqual({
      end: { x: 0, y: 0 },
      first: { x: 2, y: 2 },
      kind: "cubic",
      second: { x: 1, y: 1 },
      start: { x: 3, y: 3 },
    });
  });

  test.prop({ cubic: CUBIC, t: fc.double({ max: 1, min: 0, noNaN: true }) })(
    "traces the same points backwards",
    ({ cubic, t }) => {
      expect(cubicPoint(reversedCubic(cubic), t).x).toBeCloseTo(cubicPoint(cubic, 1 - t).x, 9);
      expect(cubicPoint(reversedCubic(cubic), t).y).toBeCloseTo(cubicPoint(cubic, 1 - t).y, 9);
    },
  );
});
