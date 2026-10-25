import { describe, expect, it } from "vitest";

import { keyDigit } from "./key-digit";

describe("keyDigit", () => {
  it.each([
    // 33 = 1 × 32 + 1.
    { digit: 1, key: 33, level: 0 },
    { digit: 1, key: 33, level: 1 },
    // 1 023 = 31 × 32 + 31.
    { digit: 31, key: 1023, level: 1 },
    // 5 has no digit at the level 2: 0.
    { digit: 0, key: 5, level: 2 },
    // 1 024 = 1 × 32² + 0 × 32 + 0.
    { digit: 1, key: 1024, level: 2 },
    { digit: 0, key: 1024, level: 0 },
  ])("[F41.AC2] reads $digit at the level $level of $key", ({ digit, key, level }) => {
    expect(keyDigit(key, level)).toBe(digit);
  });
});
