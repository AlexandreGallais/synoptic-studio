import { describe, expect, it } from "vitest";

import { keyDepth } from "./key-depth";

describe("keyDepth (EN-147)", () => {
  it.each([
    // 0 to 31: one digit in base 32.
    { depth: 1, key: 0 },
    { depth: 1, key: 31 },
    // 32 = 1 × 32 + 0 and 1 023 = 31 × 32 + 31: two digits.
    { depth: 2, key: 32 },
    { depth: 2, key: 1023 },
    // 1 024 = 32²: three digits.
    { depth: 3, key: 1024 },
  ])("[F41.AC2] gives $depth levels to the key $key", ({ depth, key }) => {
    expect(keyDepth(key)).toBe(depth);
  });
});
