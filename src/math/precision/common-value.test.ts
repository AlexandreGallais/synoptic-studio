import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { commonValue } from "./common-value";

describe("commonValue (REF-WIKI-ENGINEERING-TOLERANCE)", () => {
  it.each([
    // Every value within 1 of the first, 100: 100.
    { expected: 100, tolerance: 1, values: [100, 101, 99] },
    // 101.5 lies 1.5 from 100, past 1: none.
    { expected: undefined, tolerance: 1, values: [100, 101.5] },
    // Exact: 10 four times gives 10; one 0 among them none.
    { expected: 10, tolerance: 0, values: [10, 10, 10, 10] },
    { expected: undefined, tolerance: 0, values: [10, 0, 10, 10] },
    // One value is its own common value; none for no value.
    { expected: 7, tolerance: 0, values: [7] },
    { expected: undefined, tolerance: 0, values: [] },
    // NaN is within no tolerance of anything, itself included.
    { expected: undefined, tolerance: 1, values: [NaN] },
    { expected: undefined, tolerance: 1, values: [3, NaN] },
  ])("gives $expected for $values within $tolerance", ({ expected, tolerance, values }) => {
    expect(commonValue(values, tolerance)).toBe(expected);
  });

  it("keeps the first value, not a later one within the tolerance", () => {
    // 1 and 1 + 1e-12 lie within 1e-9 of each other: the first, 1, is kept.
    expect(commonValue([1, 1 + 1e-12], 1e-9)).toBe(1);
  });

  it("takes -0 as 0 at a tolerance of 0, keeping the first", () => {
    // |0 − (−0)| = 0 ≤ 0.
    expect(Object.is(commonValue([-0, 0], 0), -0)).toBe(true);
  });
});

describe("commonValue (properties)", () => {
  test.prop({
    count: fc.integer({ max: 8, min: 2 }),
    tolerance: fc.integer({ max: 10, min: 0 }),
    value: fc.integer({ max: 1000, min: -1000 }),
    vertex: fc.nat({ max: 7 }),
  })(
    "gives a value repeated, and none once one lies past the tolerance",
    ({ count, tolerance, value, vertex }) => {
      const values = Array.from({ length: count }, () => value);

      expect(commonValue(values, tolerance)).toBe(value);
      expect(commonValue([...values, value + tolerance], tolerance)).toBe(value);

      expect(
        commonValue(values.with(vertex % count, value + tolerance + 1), tolerance),
      ).toBeUndefined();
    },
  );
});
