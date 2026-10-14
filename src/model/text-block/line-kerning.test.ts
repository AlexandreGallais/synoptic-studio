import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_KERNING, NO_KERNING } from "../../../tests";

import { lineCodePoints } from "./line-code-points";
import { lineKerning } from "./line-kerning";

describe("lineKerning (DERIV-text-width step 2)", () => {
  it.each([
    // Only W o kerns: −37.
    { expected: -37, line: "Hello World" },
    // A V, V A, A T, T A: 4 × −152 = −608; A R kerns 0.
    { expected: -608, line: "AVATAR" },
    // o W is not W o: kerning has an order.
    { expected: 0, line: "oW" },
    // Fewer than two characters: no pair.
    { expected: 0, line: "W" },
    { expected: 0, line: "" },
  ])("sums the kerning of '$line' in Arial: $expected", ({ expected, line }) => {
    expect(lineKerning(ARIAL_KERNING, lineCodePoints(line))).toBe(expected);
  });

  test.prop({ line: fc.string({ maxLength: 20 }) })("is 0 without kerning", ({ line }) => {
    expect(lineKerning(NO_KERNING, lineCodePoints(line))).toBe(0);
  });

  test.prop({
    left: fc.stringMatching(/^[WoAVTR ]{1,10}$/u),
    right: fc.stringMatching(/^[WoAVTR ]{1,10}$/u),
  })("adds up over two pieces, plus the pair where they meet", ({ left, right }) => {
    const meeting = lineCodePoints(`${left.slice(-1)}${right.slice(0, 1)}`);

    expect(lineKerning(ARIAL_KERNING, lineCodePoints(left + right))).toBe(
      lineKerning(ARIAL_KERNING, lineCodePoints(left)) +
        lineKerning(ARIAL_KERNING, meeting) +
        lineKerning(ARIAL_KERNING, lineCodePoints(right)),
    );
  });
});
