import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { average } from "./average";

describe("average (REF-MATHWORLD-MIDPOINT)", () => {
  it("is the middle of an interval", () => {
    // (2 + 7) / 2 = 4.5.
    expect(average(2, 7)).toBe(4.5);
  });

  test.prop({
    first: fc.double({ noNaN: true, noDefaultInfinity: true, max: 1e300, min: -1e300 }),
    second: fc.double({ noNaN: true, noDefaultInfinity: true, max: 1e300, min: -1e300 }),
  })("lies between the two numbers, whatever their order", ({ first, second }) => {
    const mean = average(first, second);

    expect(mean >= Math.min(first, second) && mean <= Math.max(first, second)).toBe(true);
    expect(average(second, first)).toBe(mean);
  });
});
