import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isOnArc } from "./is-on-arc";

/** The quarter from the left (0, 50) to the top (50, 0) of the circle about (50, 50), positive. */
const QUARTER = { end: { x: 50, y: 0 }, positive: true, start: { x: 0, y: 50 } };

/** The same quarter travelled back, negative. */
const BACK = { end: QUARTER.start, positive: false, start: QUARTER.end };

describe("isOnArc (DERIV-box-overlap step 7)", () => {
  it("holds for a point of the circle on the arc", () => {
    // E − S = (50, −50), Q − S = (6.69873, −25): 50 · (−25) + 50 · 6.69873 ≈ −915.06 ≤ 0.
    expect(isOnArc(QUARTER, { x: 6.69873, y: 25 })).toBe(true);
  });

  it("fails for a point of the circle off the arc", () => {
    // Q − S = (93.30127, −25): −1250 + 4665.06 = 3415.06 > 0.
    expect(isOnArc(QUARTER, { x: 93.30127, y: 25 })).toBe(false);
  });

  it("holds at the arc's ends", () => {
    // Q = S and Q = E: the perp dot product is 0.
    expect(isOnArc(QUARTER, QUARTER.start)).toBe(true);
    expect(isOnArc(QUARTER, QUARTER.end)).toBe(true);
  });

  it("gives the same answers for the arc travelled back", () => {
    // Both the chord and σ change sign.
    expect(isOnArc(BACK, { x: 6.69873, y: 25 })).toBe(true);
    expect(isOnArc(BACK, { x: 93.30127, y: 25 })).toBe(false);
  });
});

describe("isOnArc (properties)", () => {
  test.prop({ angle: fc.double({ max: 2 * Math.PI, min: 0, noNaN: true }) })(
    "holds on the circle exactly between the angles π and 3π/2 of the quarter",
    ({ angle }) => {
      const point = { x: 50 + 50 * Math.cos(angle), y: 50 + 50 * Math.sin(angle) };

      fc.pre(Math.abs(angle - Math.PI) > 1e-6 && Math.abs(angle - (3 * Math.PI) / 2) > 1e-6);

      expect(isOnArc(QUARTER, point)).toBe(angle > Math.PI && angle < (3 * Math.PI) / 2);
    },
  );
});
