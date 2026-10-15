import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { lineCodePoints } from "./line-code-points";

describe("lineCodePoints (DERIV-text-width)", () => {
  it.each([
    { expected: [72, 233], line: "Hé" },
    { expected: [], line: "" },
    // A character beyond the basic plane is one code point, not two halves.
    { expected: [97, 0x1_f6_00], line: "a\u{1F600}" },
    // Hebrew shalom: shin, lamed, vav, final mem.
    { expected: [0x5_e9, 0x5_dc, 0x5_d5, 0x5_dd], line: "שלום" },
  ])("reads '$line' as $expected", ({ expected, line }) => {
    expect(lineCodePoints(line)).toEqual(expected);
  });

  test.prop({ line: fc.string({ maxLength: 30, unit: "grapheme" }) })(
    "gives back the line, one code point per character",
    ({ line }) => {
      expect(String.fromCodePoint(...lineCodePoints(line))).toBe(line);
    },
  );
});
