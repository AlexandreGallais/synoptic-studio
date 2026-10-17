import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { nearestInteger } from "./nearest-integer";

describe("nearestInteger (REF-MDN-MATH-ROUND, REF-CSS-VALUES-4 §5.2)", () => {
  it.each([
    // 255 · 0.5 = 127.5, a tie rounded up.
    { expected: 128, value: 127.5 },
    // 50.49 and 50.949, the reds of HSB(210°, 67 %, 60 %) and HSB(210°, 66.7 %, 60 %).
    { expected: 50, value: 50.49 },
    { expected: 51, value: 50.949 },
    // −3.5 and −2.5, ties rounded toward +∞.
    { expected: -3, value: -3.5 },
    { expected: -2, value: -2.5 },
    // Just below 0.5: 0, where ⌊0.49999999999999994 + 0.5⌋ is 1 in binary64.
    { expected: 0, value: 0.49999999999999994 },
    { expected: 7, value: 7 },
    // Quotients of integers (DERIV-color-spaces, step 2): 255 · 50 / 100 = 127.5, computed exactly,
    // a tie rounded up; 100 · 236 / 255 = 92.549…; 100 · 204 / 255 = 80; 1 / 3 and 2 / 3.
    { expected: 128, value: 12_750 / 100 },
    { expected: 93, value: 23_600 / 255 },
    { expected: 80, value: 20_400 / 255 },
    { expected: 0, value: 1 / 3 },
    { expected: 1, value: 2 / 3 },
  ])("gives $expected for $value", ({ expected, value }) => {
    expect(nearestInteger(value)).toBe(expected);
  });

  it.each([-0.5, -1e-13, -0])("gives 0, not −0, for %d", (value) => {
    expect(nearestInteger(value)).toBe(0);
  });

  test.prop({ value: fc.double({ max: 1e9, min: -1e9, noNaN: true }) })(
    "is the integer nearest the value, a tie rounded up",
    ({ value }) => {
      const rounded = nearestInteger(value);

      expect(Number.isSafeInteger(rounded)).toBe(true);
      expect(rounded - value).toBeGreaterThan(-0.5);
      expect(rounded - value).toBeLessThanOrEqual(0.5);
    },
  );
});
