import { describe, expect, it } from "vitest";

import { radiiWithTyped } from "./radii-with-typed";

describe("radiiWithTyped (AUD-046)", () => {
  it("[F44.AC4] adds -5 to each radius of a Mixed field: 10, 4, 20 give 5, 0, 15", () => {
    // 10 − 5 = 5; 4 − 5 = −1 taken at 0; 20 − 5 = 15.
    const typed = { isRelative: true, value: -5 };

    expect(radiiWithTyped([10, 4, 20], [0, 1, 2], { maxSize: 100_000, typed })).toEqual([5, 0, 15]);
  });

  it("[F44.AC4] adds +1.5 to the vertices selected alone, rounded and capped", () => {
    // 10 + 1.5 = 11.5 → 12; 99 999 + 1.5 = 100 000.5 → 100 001, capped at 100 000; 4 kept.
    const typed = { isRelative: true, value: 1.5 };

    expect(radiiWithTyped([10, 4, 99_999], [0, 2], { maxSize: 100_000, typed })).toEqual([
      12, 4, 100_000,
    ]);
  });

  it("keeps every radius when no vertex is selected", () => {
    const typed = { isRelative: true, value: 1 };

    expect(radiiWithTyped([10, 4], [], { maxSize: 100_000, typed })).toEqual([10, 4]);
  });
});
