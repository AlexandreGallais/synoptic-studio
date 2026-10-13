import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { LINE_AND_THIRDS } from "../../../tests";
import { curveFilletOf } from "../curve-fillet";

import { requestedSetbacks } from "./requested-setbacks";

import type { CornerAsk } from "./corner-ask";

/**
 * The right corner at (100, 0) of check 1a asking a radius.
 *
 * @param radius - the radius asked
 * @param isBesideCurve - whether its side after is read as a curve
 * @returns what it asks
 */
function askOf(radius: number, isBesideCurve: boolean): CornerAsk {
  const [line] = LINE_AND_THIRDS;

  return {
    angle: Math.PI / 2,
    fillet: isBesideCurve ? curveFilletOf(LINE_AND_THIRDS, radius) : undefined,
    radius,
    sides: isBesideCurve ? LINE_AND_THIRDS : [line, line],
    triple: { next: { x: 100, y: 50 }, previous: { x: 0, y: 0 }, vertex: { x: 100, y: 0 } },
  };
}

describe("requestedSetbacks (DERIV-fillet-setback, DERIV-fillet-beside-curve step 8)", () => {
  it("asks r tan(|τ|/2) on two straight sides, and the touch point's distance beside a curve", () => {
    // A right corner: tan(45°) = 1, so a radius 10 asks 10 on each straight side, 0 on the curve.
    expect(requestedSetbacks(askOf(10, false))).toEqual([
      expect.closeTo(10, 12),
      expect.closeTo(10, 12),
    ]);

    expect(requestedSetbacks(askOf(10, true))).toEqual([expect.closeTo(10, 9), 0]);
  });

  test.prop({ radius: fc.integer({ max: 50, min: 0 }) })(
    "asks as much beside a straight cubic as between two straight sides",
    ({ radius }) => {
      expect(requestedSetbacks(askOf(radius, true))[0]).toBeCloseTo(
        requestedSetbacks(askOf(radius, false))[0],
        9,
      );
    },
  );
});
