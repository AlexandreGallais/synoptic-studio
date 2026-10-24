import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { landedNudgeShift } from "./landed-nudge-shift";

import type { Point } from "../../math";

/**
 * Asserts a shift to 9 decimals on each axis.
 *
 * @param shift - the shift computed
 * @param expected - the shift computed by hand
 */
function expectShift(shift: Point, expected: Point): void {
  expect(shift.x).toBeCloseTo(expected.x, 9);
  expect(shift.y).toBeCloseTo(expected.y, 9);
}

describe("landedNudgeShift (US-089)", () => {
  it("[F46.AC1] lands the sharp turned by 30° at (6.65064, −2.5) on (8, −2) for Right", () => {
    // 6.65064 + 1 = 7.65064 rounds to 8, a shift of 1.34936; −2.5 rounds up to −2, a tie, 0.5.
    expectShift(landedNudgeShift({ x: 6.65064, y: -2.5 }, { x: 1, y: 0 }), { x: 1.34936, y: 0.5 });
  });

  it("[F46.AC1] lands it on (17, −2) for Shift + Right", () => {
    // 6.65064 + 10 = 16.65064 rounds to 17, a shift of 10.34936.
    expectShift(landedNudgeShift({ x: 6.65064, y: -2.5 }, { x: 10, y: 0 }), {
      x: 10.34936,
      y: 0.5,
    });
  });

  it("[F46.AC1] lands A′ at (4.19873, −1.65064) on (5, −3) for Up and Right held", () => {
    // (+1, −1): 5.19873 rounds to 5, a shift of 0.80127; −2.65064 to −3, a shift of −1.34936.
    expectShift(landedNudgeShift({ x: 4.19873, y: -1.65064 }, { x: 1, y: -1 }), {
      x: 0.80127,
      y: -1.34936,
    });
  });

  it("[F46.AC1] lands the dashed box of A and B at (10, −1.65064) on (11, −2) for Right", () => {
    // 10 + 1 = 11, a shift of 1; −1.65064 rounds to −2, a shift of −0.34936.
    const shift = landedNudgeShift({ x: 10, y: -1.65064 }, { x: 1, y: 0 });

    expect(shift.x).toBe(1);
    expect(shift.y).toBeCloseTo(-0.34936, 9);
  });

  it("[F46.AC1] moves an anchor on integers by exactly the nudge, as before F46", () => {
    // (10, 20) and Shift + Up: (0, −10).
    expect(landedNudgeShift({ x: 10, y: 20 }, { x: 0, y: -10 })).toEqual({ x: 0, y: -10 });
  });

  it("[F46.AC1] moves an anchor on integers but for a rounding error by exactly the nudge", () => {
    // A rounded rectangle at (0, 0), its straight box at −1.8 × 10⁻¹⁵: landed, the shift would be
    // 1 + 1.8 × 10⁻¹⁵ and 1.8 × 10⁻¹⁵, within EPSILON of (1, 0); its position stays whole.
    const shift = landedNudgeShift({ x: -1.8e-15, y: -1.8e-15 }, { x: 1, y: 0 });

    expect(shift.x).toBe(1);
    expect(shift.y).toBe(0);
  });

  test.prop({
    anchor: fc.record({
      x: fc.double({ max: 1e5, min: -1e5, noNaN: true }),
      y: fc.double({ max: 1e5, min: -1e5, noNaN: true }),
    }),
    nudge: fc.record({
      x: fc.constantFrom(-10, -1, 0, 1, 10),
      y: fc.constantFrom(-10, -1, 0, 1, 10),
    }),
  })(
    "lands the anchor on integers, within half a unit of where the nudge took it",
    ({ anchor, nudge }) => {
      const shift = landedNudgeShift(anchor, nudge);

      for (const axis of ["x", "y"] as const) {
        const landed = anchor[axis] + shift[axis];

        // Within EPSILON, an anchor on integers but for an error keeps it, moved by the nudge.
        expect(Math.abs(landed - Math.round(landed))).toBeLessThanOrEqual(2e-9);
        expect(Math.abs(shift[axis] - nudge[axis])).toBeLessThanOrEqual(0.5 + 1e-9);
      }
    },
  );
});
