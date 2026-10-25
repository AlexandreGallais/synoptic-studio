import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { landedShift } from "./landed-shift";

describe("landedShift", () => {
  it("[F39.AC2] lands A's box at (10, 20), dragged by (30.6, 19.4), on (41, 39)", () => {
    // 10 + 30.6 = 40.6 rounds to 41, 20 + 19.4 = 39.4 to 39: a shift of (31, 19).
    expect(landedShift({ x: 10, y: 20 }, { x: 30.6, y: 19.4 })).toEqual({ x: 31, y: 19 });
  });

  it("[F39.AC2] lands B's straight box, turned by 30°, on (205, 2), its position keeping fractions", () => {
    // 194.19873 + 10.6 = 204.79873 rounds to 205, a shift of 10.80127; −1.65064 + 3.2 = 1.54936
    // rounds to 2, a shift of 3.65064.
    const shift = landedShift({ x: 194.19873, y: -1.65064 }, { x: 10.6, y: 3.2 });

    expect(shift.x).toBeCloseTo(10.80127, 9);
    expect(shift.y).toBeCloseTo(3.65064, 9);
  });

  it("[F39.AC2] lands the dashed box of A and B at (10, −1.65064) on (21, 2)", () => {
    // 10 + 10.6 = 20.6 rounds to 21, a shift of 11; −1.65064 + 3.2 = 1.54936 to 2, 3.65064.
    const shift = landedShift({ x: 10, y: -1.65064 }, { x: 10.6, y: 3.2 });

    expect(shift.x).toBe(11);
    expect(shift.y).toBeCloseTo(3.65064, 9);
  });

  it("[F39.AC2] rounds a tie up, toward +∞, as a click places a shape", () => {
    // 10 + 0.5 = 10.5 rounds to 11; 20 − 0.5 = 19.5 rounds to 20, a shift of 0.
    expect(landedShift({ x: 10, y: 20 }, { x: 0.5, y: -0.5 })).toEqual({ x: 1, y: 0 });
  });

  it("[F39.AC2] lands a small tremble, (0.7, 0.2), a whole unit away", () => {
    // 10.7 rounds to 11, 20.2 to 20.
    expect(landedShift({ x: 10, y: 20 }, { x: 0.7, y: 0.2 })).toEqual({ x: 1, y: 0 });
  });

  it("keeps an anchor on integers with no offset where it is, never −0", () => {
    expect(landedShift({ x: -3, y: 0 }, { x: 0, y: 0 })).toEqual({ x: 0, y: 0 });
    expect(Object.is(landedShift({ x: 0, y: 0 }, { x: -0.2, y: -0 }).x, 0)).toBe(true);
  });

  test.prop({
    anchor: fc.record({
      x: fc.double({ max: 1e5, min: -1e5, noNaN: true }),
      y: fc.double({ max: 1e5, min: -1e5, noNaN: true }),
    }),
    offset: fc.record({
      x: fc.double({ max: 1e5, min: -1e5, noNaN: true }),
      y: fc.double({ max: 1e5, min: -1e5, noNaN: true }),
    }),
  })(
    "lands the anchor on integers, within half a unit of where the pointer took it",
    ({ anchor, offset }) => {
      const shift = landedShift(anchor, offset);

      for (const axis of ["x", "y"] as const) {
        const landed = anchor[axis] + shift[axis];

        expect(Math.abs(landed - Math.round(landed))).toBeLessThan(1e-9);
        expect(Math.abs(shift[axis] - offset[axis])).toBeLessThanOrEqual(0.5 + 1e-9);
      }
    },
  );
});
