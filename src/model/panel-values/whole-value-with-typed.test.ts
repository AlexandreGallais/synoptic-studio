import { describe, expect, it } from "vitest";

import { wholeValueWithTyped } from "./whole-value-with-typed";

describe("wholeValueWithTyped (AUD-046)", () => {
  it.each([
    // 1.5 → 2, a tie rounded up; 10 + 1.5 = 11.5 → 12; 10 − 2.5 = 7.5 → 8; −0.5 → 0, never −0.
    { expected: 2, own: 120, typed: { isRelative: false, value: 1.5 } },
    { expected: 12, own: 10, typed: { isRelative: true, value: 1.5 } },
    { expected: 8, own: 10, typed: { isRelative: true, value: -2.5 } },
    { expected: 0, own: 0, typed: { isRelative: true, value: -0.5 } },
    // 4.19873 + 0.3 = 4.49873 → 4: the result rounded, not the value typed alone.
    { expected: 4, own: 4.19873, typed: { isRelative: true, value: 0.3 } },
  ])(
    "[F44.AC4] gives $expected from $own and $typed.value, a fraction typed",
    ({ expected, own, typed }) => {
      expect(wholeValueWithTyped(own, typed)).toBe(expected);
    },
  );

  it("[F44.AC4] keeps a fraction of the own value for an integer typed: 86.60254 + 10", () => {
    // A converted triangle's height of 50√3 keeps its fraction (Q27).
    expect(wholeValueWithTyped(50 * Math.sqrt(3), { isRelative: true, value: 10 })).toBe(
      50 * Math.sqrt(3) + 10,
    );
  });
});
