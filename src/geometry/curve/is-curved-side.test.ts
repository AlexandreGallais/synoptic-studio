import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";

import { isCurvedSide } from "./is-curved-side";

import type { Vector } from "../../math";
import type { CurveCorner } from "../handles";

/**
 * A corner with given handles.
 *
 * @param isOn - handles mode
 * @param before - handle towards the previous point
 * @param after - handle towards the next point
 * @returns the corner at (0, 0), radius 10
 */
function corner(isOn: boolean, before: Vector, after: Vector): CurveCorner {
  return {
    handles: { after, before, isOn, mode: "none", tension: 0 },
    point: { x: 0, y: 0 },
    radius: 10,
  };
}

/** A zero handle. */
const ZERO = { x: 0, y: 0 };

describe("isCurvedSide (ADR-0046)", () => {
  it("[F12.AC3] curves a side by the start's handle after or the end's handle before, on and not zero", () => {
    expect(isCurvedSide(corner(true, ZERO, { x: 30, y: -15 }), corner(false, ZERO, ZERO))).toBe(
      true,
    );

    expect(isCurvedSide(corner(false, ZERO, ZERO), corner(true, { x: -30, y: 0 }, ZERO))).toBe(
      true,
    );
  });

  it("leaves it straight when the handles are off, zero, or on the other sides", () => {
    // Off: the vectors are kept but not drawn; on but zero: a straight side (REF-FIGMA-VECTOR-NETWORK).
    expect(
      isCurvedSide(corner(false, ZERO, { x: 30, y: 0 }), corner(false, { x: 1, y: 1 }, ZERO)),
    ).toBe(false);

    expect(isCurvedSide(corner(true, ZERO, ZERO), corner(true, ZERO, ZERO))).toBe(false);

    expect(
      isCurvedSide(corner(true, { x: 5, y: 5 }, ZERO), corner(true, ZERO, { x: 5, y: 5 })),
    ).toBe(false);
  });

  test.prop({ after: integerPoint(100), before: integerPoint(100) })(
    "never curves a side whose two ends are off",
    ({ after, before }) => {
      expect(isCurvedSide(corner(false, before, after), corner(false, before, after))).toBe(false);
    },
  );
});
