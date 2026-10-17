import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ceiling } from "./ceiling";

describe("ceiling (REF-ECMA-262-MATH-CEIL)", () => {
  it.each([
    // "Pump P-101" in Inter 16: a block 87.953125 × 19.359375 rounded up to 88 × 20.
    { expected: 88, value: 87.953125 },
    { expected: 20, value: 19.359375 },
    // Two lines: 2 × 19.359375 = 38.71875, rounded up to 39.
    { expected: 39, value: 38.71875 },
    // Just above an integer: one more; an integer stays.
    { expected: 89, value: 88.0000001 },
    { expected: 88, value: 88 },
    { expected: 0, value: 0 },
    // Negative values round toward +∞: −1.5 to −1.
    { expected: -1, value: -1.5 },
    { expected: -2, value: -2 },
  ])("gives $expected for $value", ({ expected, value }) => {
    expect(ceiling(value)).toBe(expected);
  });

  it.each([-0.5, -1e-9, -0])("gives 0, not −0, for %d", (value) => {
    expect(ceiling(value)).toBe(0);
  });

  test.prop({ value: fc.double({ max: 1e9, min: -1e9, noNaN: true }) })(
    "is the smallest integer not less than the value",
    ({ value }) => {
      const rounded = ceiling(value);

      expect(Number.isSafeInteger(rounded)).toBe(true);
      expect(rounded).toBeGreaterThanOrEqual(value);
      expect(rounded - 1).toBeLessThan(value);
    },
  );
});
