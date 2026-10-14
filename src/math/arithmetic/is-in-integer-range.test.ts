import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isInIntegerRange } from "./is-in-integer-range";

describe("isInIntegerRange (REF-WIKI-INTERVAL)", () => {
  it.each([
    // Both ends are included.
    { isIn: true, first: 1, last: 5, value: 1 },
    { isIn: true, first: 1, last: 5, value: 5 },
    { isIn: true, first: 1, last: 5, value: 3 },
    { isIn: false, first: 1, last: 5, value: 0 },
    { isIn: false, first: 1, last: 5, value: 6 },
    // A fraction between the ends is no integer of the interval.
    { isIn: false, first: 1, last: 5, value: 2.5 },
    { isIn: false, first: 1, last: 5, value: NaN },
    { isIn: false, first: -10, last: 10, value: Infinity },
    // An interval of one integer holds it; the last before the first holds none.
    { isIn: true, first: 7, last: 7, value: 7 },
    { isIn: false, first: 5, last: 4, value: 5 },
  ])("tells whether $value is in $first .. $last: $isIn", ({ first, isIn, last, value }) => {
    expect(isInIntegerRange(value, first, last)).toBe(isIn);
  });

  test.prop({ first: fc.integer({ max: 1000, min: -1000 }), size: fc.nat({ max: 200 }) })(
    "holds exactly the integers integerRange lists, and no neighbour outside",
    ({ first, size }) => {
      const last = first + size;

      expect(isInIntegerRange(first, first, last)).toBe(true);
      expect(isInIntegerRange(last, first, last)).toBe(true);
      expect(isInIntegerRange(first - 1, first, last)).toBe(false);
      expect(isInIntegerRange(last + 1, first, last)).toBe(false);
    },
  );
});
