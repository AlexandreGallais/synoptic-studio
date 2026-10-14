import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { maximum } from "./maximum";

describe("maximum (REF-WIKI-MAXIMUM)", () => {
  it.each([
    { expected: 5, floor: 0, values: [1, 5, 3] },
    // No number: the floor.
    { expected: 0, floor: 0, values: [] },
    // Every number under the floor: the floor.
    { expected: 0, floor: 0, values: [-3, -1] },
    { expected: -1, floor: -Infinity, values: [-3, -1] },
  ])("gives $expected for $values over $floor", ({ expected, floor, values }) => {
    expect(maximum(values, floor)).toBe(expected);
  });

  it("takes 200 000 numbers, past what a call's arguments may hold", () => {
    const values = Array.from({ length: 200_000 }, (unused, index) => index);

    expect(maximum(values, 0)).toBe(199_999);
  });

  test.prop({ floor: fc.integer(), values: fc.array(fc.integer()) })(
    "is at least the floor and every number, and one of them",
    ({ floor, values }) => {
      const greatest = maximum(values, floor);

      expect([floor, ...values].every((value) => value <= greatest)).toBe(true);
      expect([floor, ...values]).toContain(greatest);
    },
  );
});
