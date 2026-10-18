import { describe, expect, it } from "vitest";

import { integerRange } from "../../math";

import { alphaToPercent } from "./alpha-to-percent";

describe("alphaToPercent (DERIV-color-spaces, step 2)", () => {
  it.each([
    // ⌊(200 · 204 + 255) / 510⌋ = ⌊80.5⌋ = 80: `cc`, CSS's #0000ffcc at 80 %.
    { alpha: 204, expected: 80 },
    // ⌊(200 · 236 + 255) / 510⌋ = ⌊93.05⌋ = 93: `ec`, 92.549… % rounded.
    { alpha: 236, expected: 93 },
    // ⌊(200 · 128 + 255) / 510⌋ = ⌊50.696⌋ = 50: `80`.
    { alpha: 128, expected: 50 },
    // ⌊(200 · 1 + 255) / 510⌋ = 0: 0.39 % rounded down.
    { alpha: 1, expected: 0 },
    // ⌊(200 · 2 + 255) / 510⌋ = 1: 0.78 % rounded up.
    { alpha: 2, expected: 1 },
    { alpha: 0, expected: 0 },
    { alpha: 255, expected: 100 },
  ])("[F16.AC1] gives $expected % for the byte $alpha", ({ alpha, expected }) => {
    expect(alphaToPercent(alpha)).toBe(expected);
  });

  it("gives every byte a whole percent from 0 to 100, never going down", () => {
    const percents = integerRange(0, 255).map((alpha) => alphaToPercent(alpha));

    expect(percents.every((percent, index) => percent >= (percents[index - 1] ?? 0))).toBe(true);
    expect(new Set(percents)).toHaveProperty("size", 101);
  });
});
