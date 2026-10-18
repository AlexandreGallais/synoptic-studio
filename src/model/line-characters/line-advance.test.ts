import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_METRICS } from "../../../tests";

import { lineAdvance } from "./line-advance";
import { lineCodePoints } from "./line-code-points";

describe("lineAdvance (DERIV-text-width step 2)", () => {
  it.each([
    // H 1479 + e 1139 + 3 × l 455 + 2 × o 1139 + space 569 + W 1933 + r 682 + d 1139.
    { expected: 10_584, line: "Hello World" },
    // 3 × A 1366 + V 1366 + T 1251 + R 1479 = 8194.
    { expected: 8194, line: "AVATAR" },
    { expected: 1479, line: "H" },
    { expected: 0, line: "" },
    // Z is not among the advances: it takes glyph 0's, 1536; H Z = 1479 + 1536.
    { expected: 3015, line: "HZ" },
  ])("sums the advances of '$line' in Arial: $expected", ({ expected, line }) => {
    expect(lineAdvance(ARIAL_METRICS, lineCodePoints(line))).toBe(expected);
  });

  test.prop({
    left: fc.stringMatching(/^[HeloWrdAVTR ]{0,12}$/u),
    right: fc.stringMatching(/^[HeloWrdAVTR ]{0,12}$/u),
  })("adds up over two pieces of a line", ({ left, right }) => {
    expect(lineAdvance(ARIAL_METRICS, lineCodePoints(left + right))).toBe(
      lineAdvance(ARIAL_METRICS, lineCodePoints(left)) +
        lineAdvance(ARIAL_METRICS, lineCodePoints(right)),
    );
  });
});
