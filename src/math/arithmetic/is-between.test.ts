import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isBetween } from "./is-between";

describe("isBetween (DERIV-box-overlap step 1)", () => {
  it("holds inside the interval and on both ends", () => {
    // 0 ≤ 5 ≤ 10; 0 ≤ 0; 10 ≤ 10.
    expect(isBetween(5, 0, 10)).toBe(true);
    expect(isBetween(0, 0, 10)).toBe(true);
    expect(isBetween(10, 0, 10)).toBe(true);
  });

  it("fails outside the interval", () => {
    expect(isBetween(-1, 0, 10)).toBe(false);
    expect(isBetween(11, 0, 10)).toBe(false);
  });

  it("takes the ends in either order", () => {
    // min(10, 0) = 0, max(10, 0) = 10.
    expect(isBetween(5, 10, 0)).toBe(true);
    expect(isBetween(11, 10, 0)).toBe(false);
  });

  it("holds for an interval of one point at that point only", () => {
    expect(isBetween(3, 3, 3)).toBe(true);
    expect(isBetween(3.5, 3, 3)).toBe(false);
  });
});

describe("isBetween (properties)", () => {
  test.prop({
    first: fc.double({ noNaN: true }),
    second: fc.double({ noNaN: true }),
    value: fc.double({ noNaN: true }),
  })(
    "gives the same answer with the ends exchanged, and holds at each end",
    ({ first, second, value }) => {
      expect(isBetween(value, first, second)).toBe(isBetween(value, second, first));
      expect(isBetween(first, first, second)).toBe(true);
    },
  );
});
