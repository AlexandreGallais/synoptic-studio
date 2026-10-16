import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { byteToFraction } from "./byte-to-fraction";
import { fractionToByte } from "./fraction-to-byte";

describe("byteToFraction (REF-CSS-COLOR-4 §5.2)", () => {
  it.each([
    // 51 / 255 = 0.2, 102 / 255 = 0.4, 0 and 255 the ends.
    { byte: 51, expected: 0.2 },
    { byte: 102, expected: 0.4 },
    { byte: 0, expected: 0 },
    { byte: 255, expected: 1 },
  ])("gives byte $byte as $expected", ({ byte, expected }) => {
    expect(byteToFraction(byte)).toBe(expected);
  });

  test.prop({ byte: fc.integer({ max: 255, min: 0 }) })(
    "gives back its byte through fractionToByte",
    ({ byte }) => {
      expect(fractionToByte(byteToFraction(byte))).toBe(byte);
    },
  );
});
