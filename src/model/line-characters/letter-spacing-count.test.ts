import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { letterSpacingCount } from "./letter-spacing-count";

describe("letterSpacingCount (REF-CSS-TEXT-3, DERIV-text-width step 3)", () => {
  it.each([
    // "Hello World": 11 characters, 10 pairs.
    { characterCount: 11, expected: 10 },
    // A single character: no letter spacing at the line's start or end.
    { characterCount: 1, expected: 0 },
    { characterCount: 0, expected: 0 },
  ])(
    "adds letter spacing $expected times in $characterCount characters",
    ({ characterCount, expected }) => {
      expect(letterSpacingCount(characterCount)).toBe(expected);
    },
  );

  test.prop({ characterCount: fc.nat(1000) })(
    "adds it once more for each character past the first",
    ({ characterCount }) => {
      expect(letterSpacingCount(characterCount + 1)).toBe(characterCount);
    },
  );
});
