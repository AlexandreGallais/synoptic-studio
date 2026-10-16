import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { progressionTermsUpTo } from "./progression-terms-up-to";

describe("progressionTermsUpTo (REF-WIKI-ARITHMETIC-PROGRESSION)", () => {
  it.each([
    // 16, 36, 56, 76: three terms up to 60, ⌊44 / 20⌋ + 1.
    { bound: 60, difference: 20, expected: 3, first: 16 },
    // A bound on a term counts it: 16, 36, 56 up to 56.
    { bound: 56, difference: 20, expected: 3, first: 16 },
    // The first term alone: up to 35, ⌊19 / 20⌋ + 1.
    { bound: 35, difference: 20, expected: 1, first: 16 },
    { bound: 16, difference: 20, expected: 1, first: 16 },
    // 0.1 by 0.1 up to 0.3: (0.3 − 0.1) / 0.1 is 1.9999999999999998, the ε counts the third term.
    { bound: 0.3, difference: 0.1, expected: 3, first: 0.1 },
  ])("counts $expected terms from $first by $difference up to $bound", (progression) => {
    const { bound, difference, first } = progression;

    expect(progressionTermsUpTo(first, difference, bound)).toBe(progression.expected);
  });

  test.prop({
    bound: fc.integer({ max: 10_000, min: 0 }),
    difference: fc.integer({ max: 500, min: 1 }),
    first: fc.integer({ max: 10_000, min: 0 }),
  })("counts the last term not above the bound, the next one above it", (progression) => {
    const { difference } = progression;
    const first = Math.min(progression.first, progression.bound);
    const n = progressionTermsUpTo(first, difference, progression.bound);

    // aₙ = a₁ + (n − 1) · d ≤ b < aₙ₊₁ = a₁ + n · d.
    expect(n).toBeGreaterThanOrEqual(1);
    expect(first + (n - 1) * difference).toBeLessThanOrEqual(progression.bound);
    expect(first + n * difference).toBeGreaterThan(progression.bound);
  });
});
