import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { LINE_AND_QUARTER, LINE_AND_THIRDS, TWO_CURVES } from "../../../tests";
import { norm, subtract } from "../../math";

import { curveFilletOf } from "./curve-fillet-of";

describe("curveFilletOf (DERIV-fillet-beside-curve, checks 1a, 2 and 3)", () => {
  it("[F12.AC7] draws today's fillet beside a straight cubic: from (90, 0) to (100, 10)", () => {
    const { arc } = curveFilletOf(LINE_AND_THIRDS, 10);

    expect(arc.radius).toBe(10);
    expect(arc.start.x).toBeCloseTo(90, 9);
    expect(arc.end.y).toBeCloseTo(10, 9);
    // Coming right along y = 0, then down: a positive turn, as `turningAngle`.
    expect(arc.positive).toBe(true);
  });

  it("[F12.AC7] touches the quarter circle at its middle for the radius 100(√2 − 1)", () => {
    // B(½) lies on the circle; its normal points at the centre (100√2 − 200, 100√2 − 100).
    const fillet = curveFilletOf(LINE_AND_QUARTER, 100 * (Math.SQRT2 - 1));

    expect(fillet.after).toBeCloseTo(0.5, 9);
    expect(fillet.arc.center.x).toBeCloseTo(100 * Math.SQRT2 - 200, 9);
    expect(fillet.arc.center.y).toBeCloseTo(100 * Math.SQRT2 - 100, 9);
  });

  it("[F12.AC7] draws an arc between two curved sides", () => {
    expect(curveFilletOf(TWO_CURVES, 10).arc.center.x).toBeCloseTo(90, 9);
  });

  it("draws an arc without length at the corner for radius 0", () => {
    expect(curveFilletOf(LINE_AND_THIRDS, 0)).toEqual({
      after: 0,
      arc: {
        center: { x: 100, y: 0 },
        end: { x: 100, y: 0 },
        kind: "arc",
        positive: true,
        radius: 0,
        start: { x: 100, y: 0 },
      },
      before: 0,
    });
  });

  test.prop({ radius: fc.integer({ max: 50, min: 1 }) })(
    "starts and ends its arc the radius from its centre",
    ({ radius }) => {
      const { arc } = curveFilletOf(LINE_AND_THIRDS, radius);

      expect(norm(subtract(arc.start, arc.center))).toBeCloseTo(radius, 9);
      expect(norm(subtract(arc.end, arc.center))).toBeCloseTo(radius, 9);
    },
  );
});
