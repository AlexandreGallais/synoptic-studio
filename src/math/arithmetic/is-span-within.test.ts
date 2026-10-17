import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isSpanWithin } from "./is-span-within";

describe("isSpanWithin (REF-WIKI-INTERVAL)", () => {
  it.each([
    // 4 .. 7 inside 0 .. 9.
    { end: 10, isWithin: true, size: 4, start: 4 },
    // 6 .. 9 ends on the last integer, 9.
    { end: 10, isWithin: true, size: 4, start: 6 },
    // 7 .. 10 passes 9.
    { end: 10, isWithin: false, size: 4, start: 7 },
    // −1 .. 2 starts before 0.
    { end: 10, isWithin: false, size: 4, start: -1 },
    // An empty span at the end.
    { end: 10, isWithin: true, size: 0, start: 10 },
  ])("tells $start + $size within 0 .. $end − 1: $isWithin", ({ end, isWithin, size, start }) => {
    expect(isSpanWithin(start, size, end)).toBe(isWithin);
  });

  test.prop({
    end: fc.nat({ max: 1000 }),
    size: fc.nat({ max: 1000 }),
    start: fc.integer({ max: 1000, min: -10 }),
  })(
    "agrees with every integer of the span lying in the range, its start not past the end",
    ({ end, size, start }) => {
      const integers = Array.from({ length: size }, (unused, index) => start + index);

      expect(isSpanWithin(start, size, end)).toBe(
        start >= 0 && start <= end && integers.every((integer) => integer < end),
      );
    },
  );
});
