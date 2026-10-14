import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerRange } from "./integer-range";

describe("integerRange (REF-WIKI-INTERVAL)", () => {
  it.each([
    { expected: [3, 4, 5], first: 3, last: 5 },
    // An interval of one integer includes it.
    { expected: [7], first: 7, last: 7 },
    { expected: [-2, -1, 0], first: -2, last: 0 },
    // The last before the first: no integer.
    { expected: [], first: 5, last: 4 },
  ])("lists $first .. $last as $expected", ({ expected, first, last }) => {
    expect(integerRange(first, last)).toEqual(expected);
  });

  test.prop({ first: fc.integer({ max: 1000, min: -1000 }), size: fc.nat({ max: 200 }) })(
    "holds size integers, each one more than the one before, from the first",
    ({ first, size }) => {
      const range = integerRange(first, first + size - 1);

      expect(range).toHaveLength(size);
      expect(range.every((integer, index) => integer === first + index)).toBe(true);
    },
  );
});
