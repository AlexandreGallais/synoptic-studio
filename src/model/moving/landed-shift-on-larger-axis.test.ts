import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { landedShiftOnLargerAxis } from "./landed-shift-on-larger-axis";

describe("landedShiftOnLargerAxis (AUD-041)", () => {
  it("[F39.AC2] lands A at (10, 20), dragged by (30.6, 19.4) with Shift, on (41, 20)", () => {
    // Across, the larger: 10 + 30.6 = 40.6 rounds to 41, a shift of 31; down kept still, 0.
    expect(landedShiftOnLargerAxis({ x: 10, y: 20 }, { x: 30.6, y: 19.4 })).toEqual({
      x: 31,
      y: 0,
    });
  });

  it("[F39.AC2] keeps the dashed box of A and B at y −1.65064 still, only across landed", () => {
    // 10 + 30.6 = 40.6 rounds to 41, a shift of 31; y −1.65064 not landed on −2.
    expect(landedShiftOnLargerAxis({ x: 10, y: -1.65064 }, { x: 30.6, y: 19.4 })).toEqual({
      x: 31,
      y: 0,
    });
  });

  it("[F39.AC2] lands down alone when down is larger, x 194.19873 kept still", () => {
    // Down, the larger: −1.65064 + 12.5 = 10.84936 rounds to 11, a shift of 12.65064.
    const shift = landedShiftOnLargerAxis({ x: 194.19873, y: -1.65064 }, { x: -3, y: 12.5 });

    expect(shift.x).toBe(0);
    expect(shift.y).toBeCloseTo(12.65064, 9);
  });

  it("[F39.AC2] lands a tie, (20, −20), across, A going to (30, 20)", () => {
    // 10 + 20 = 30, a shift of 20; down kept still.
    expect(landedShiftOnLargerAxis({ x: 10, y: 20 }, { x: 20, y: -20 })).toEqual({ x: 20, y: 0 });
  });

  it("lands B's straight box across only, its x 194.19873 on 205", () => {
    // 194.19873 + 10.6 = 204.79873 rounds to 205, a shift of 10.80127.
    const shift = landedShiftOnLargerAxis({ x: 194.19873, y: -1.65064 }, { x: 10.6, y: 3.2 });

    expect(shift.x).toBeCloseTo(10.80127, 9);
    expect(shift.y).toBe(0);
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
    "moves on one axis alone, landing the anchor there on integers within half a unit",
    ({ anchor, offset }) => {
      const shift = landedShiftOnLargerAxis(anchor, offset);
      const isAcross = Math.abs(offset.x) >= Math.abs(offset.y);
      const axis = isAcross ? "x" : "y";
      const landed = anchor[axis] + shift[axis];

      expect(isAcross ? shift.y : shift.x).toBe(0);
      expect(Math.abs(landed - Math.round(landed))).toBeLessThan(1e-9);
      expect(Math.abs(shift[axis] - offset[axis])).toBeLessThanOrEqual(0.5 + 1e-9);
    },
  );
});
