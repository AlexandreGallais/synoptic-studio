import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { minimum } from "./minimum";

describe("minimum (REF-WIKI-MAXIMUM)", () => {
  it.each([
    { ceiling: 9, expected: 1, values: [3, 1, 5] },
    // No number: the ceiling.
    { ceiling: 9, expected: 9, values: [] },
    // Every number over the ceiling: the ceiling.
    { ceiling: 0, expected: 0, values: [3, 1] },
    { ceiling: Infinity, expected: 1, values: [3, 1] },
  ])("gives $expected for $values under $ceiling", ({ ceiling, expected, values }) => {
    expect(minimum(values, ceiling)).toBe(expected);
  });

  it("takes 200 000 numbers, past what a call's arguments may hold", () => {
    const values = Array.from({ length: 200_000 }, (unused, index) => index);

    expect(minimum(values, Infinity)).toBe(0);
  });

  test.prop({ ceiling: fc.integer(), values: fc.array(fc.integer()) })(
    "is at most the ceiling and every number, and one of them",
    ({ ceiling, values }) => {
      const least = minimum(values, ceiling);

      expect([ceiling, ...values].every((value) => value >= least)).toBe(true);
      expect([ceiling, ...values]).toContain(least);
    },
  );
});
