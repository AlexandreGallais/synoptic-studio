import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { degreesInTurn } from "./degrees-in-turn";

describe("degreesInTurn (DERIV-turned-shape step 1, REF-MDN-REMAINDER)", () => {
  it.each([
    // 370 = 360 + 10.
    { angle: 370, expected: 10 },
    // −90 % 360 is −90 (sign of the dividend); + 360 gives 270.
    { angle: -90, expected: 270 },
    { angle: 360, expected: 0 },
    { angle: 45, expected: 45 },
    { angle: 0, expected: 0 },
    // −720 % 360 is −0; −0 + 360 = 360; 360 % 360 = 0.
    { angle: -720, expected: 0 },
    { angle: 359, expected: 359 },
  ])("[F04.AC1] keeps $angle as $expected", ({ angle, expected }) => {
    expect(degreesInTurn(angle)).toBe(expected);
  });

  it("[F04.AC1] gives a positive zero for −360", () => {
    expect(Object.is(degreesInTurn(-360), 0)).toBe(true);
  });

  test.prop({ angle: fc.integer({ max: 1e9, min: -1e9 }) })(
    "is an integer from 0 to 359, the same direction as the angle typed",
    ({ angle }) => {
      const kept = degreesInTurn(angle);

      expect(Number.isSafeInteger(kept)).toBe(true);
      expect(kept).toBeGreaterThanOrEqual(0);
      expect(kept).toBeLessThan(360);
      expect((angle - kept) % 360).toBeCloseTo(0, 12);
    },
  );
});
