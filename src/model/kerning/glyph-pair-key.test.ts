import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { glyphPairKey } from "./glyph-pair-key";

describe("glyphPairKey (REF-OT-KERN)", () => {
  it.each([
    // 1 × 65 536 + 2.
    { expected: 65_538, first: 1, second: 2 },
    { expected: 0, first: 0, second: 0 },
    // 65 535 × 65 536 + 65 535 = 2³² − 1.
    { expected: 4_294_967_295, first: 65_535, second: 65_535 },
  ])("keys $first then $second as $expected", ({ expected, first, second }) => {
    expect(glyphPairKey(first, second)).toBe(expected);
  });

  test.prop({
    first: fc.integer({ max: 65_535, min: 0 }),
    second: fc.integer({ max: 65_535, min: 0 }),
  })(
    "gives the first glyph as the high-order word and the second as the low-order one",
    ({ first, second }) => {
      const key = glyphPairKey(first, second);

      expect(Math.floor(key / 65_536)).toBe(first);
      expect(key % 65_536).toBe(second);
    },
  );
});
