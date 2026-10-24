import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { fieldArrowShift } from "./field-arrow-shift";

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

describe("fieldArrowShift (US-089)", () => {
  it("[F46.AC1] lands the sharp turned by 30° at (6.65064, −2.5) on (8, −2) for Up in X", () => {
    // As Right: 6.65064 + 1 = 7.65064 rounds to 8, a shift of 1.34936; −2.5 + 0 = −2.5 rounds up
    // to −2, a tie, a shift of 0.5.
    expectShift(fieldArrowShift({ x: 6.65064, y: -2.5 }, "x", 1), { x: 1.34936, y: 0.5 });
  });

  it("[F46.AC1] lands it on (17, −2) for Shift + Up in X", () => {
    // 6.65064 + 10 = 16.65064 rounds to 17, a shift of 10.34936; −2.5 to −2, 0.5.
    expectShift(fieldArrowShift({ x: 6.65064, y: -2.5 }, "x", 10), { x: 10.34936, y: 0.5 });
  });

  it("[F46.AC1] lands it on (7, −1) for Up in Y, as Down on the drawing", () => {
    // −2.5 + 1 = −1.5 rounds up to −1, a tie, a shift of 1.5; 6.65064 + 0 rounds to 7, 0.34936.
    expectShift(fieldArrowShift({ x: 6.65064, y: -2.5 }, "y", 1), { x: 0.34936, y: 1.5 });
  });

  it("[F46.AC1] lands A turned by 30°, at (4.19873, −1.65064), on (3, −2) for Down in X", () => {
    // As Left: 4.19873 − 1 = 3.19873 rounds to 3, a shift of −1.19873; −1.65064 rounds to −2,
    // a shift of −0.34936.
    expectShift(fieldArrowShift({ x: 4.19873, y: -1.65064 }, "x", -1), {
      x: -1.19873,
      y: -0.34936,
    });
  });

  it("[F46.AC1] moves a shape on integers by exactly the step, the other axis kept", () => {
    // (10, 20): Up in X by 1, across; Shift + Down in Y by −10, up.
    expect(fieldArrowShift({ x: 10, y: 20 }, "x", 1)).toEqual({ x: 1, y: 0 });
    expect(fieldArrowShift({ x: 10, y: 20 }, "y", -10)).toEqual({ x: 0, y: -10 });
  });

  it("[F46.AC1] rounds a tie up on both axes, toward +∞", () => {
    // (0.5, 0.5), Up in X: 1.5 rounds to 2, a shift of 1.5; 0.5 rounds to 1, a shift of 0.5.
    expect(fieldArrowShift({ x: 0.5, y: 0.5 }, "x", 1)).toEqual({ x: 1.5, y: 0.5 });
  });

  test.prop({
    anchor: fc.record({
      x: fc.double({ max: 1e5, min: -1e5, noNaN: true }),
      y: fc.double({ max: 1e5, min: -1e5, noNaN: true }),
    }),
    axis: fc.constantFrom("x" as const, "y" as const),
    step: fc.constantFrom(-10, -1, 1, 10),
  })(
    "lands the anchor on integers, the step's axis moved toward it, within half a unit",
    ({ anchor, axis, step }) => {
      const shift = fieldArrowShift(anchor, axis, step);
      const other = axis === "x" ? "y" : "x";

      for (const each of ["x", "y"] as const) {
        const landed = anchor[each] + shift[each];

        expect(Math.abs(landed - Math.round(landed))).toBeLessThan(1e-9);
      }

      expect(Math.sign(shift[axis])).toBe(Math.sign(step));
      expect(Math.abs(shift[axis] - step)).toBeLessThanOrEqual(0.5 + 1e-9);
      expect(Math.abs(shift[other])).toBeLessThanOrEqual(0.5 + 1e-9);
    },
  );
});
