import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { measuredCharacters } from "./measured-characters";

describe("measuredCharacters (REF-CSS-TEXT-3, REF-ECMA-262-NORMALIZE)", () => {
  it.each([
    { expected: "Hello", line: "Hello" },
    // e and a combining acute compose into é (U+00E9).
    { expected: "\u{E9}", line: "e\u{301}" },
    // q has no composed form with an acute: kept as typed.
    { expected: "q\u{301}", line: "q\u{301}" },
    // A zero-width space, a zero-width joiner and a soft hyphen (Cf) are left out.
    { expected: "AB", line: "A\u{200B}B" },
    { expected: "ab", line: "a\u{200D}\u{AD}b" },
    { expected: "", line: "" },
  ])("measures '$line' as '$expected'", ({ expected, line }) => {
    expect(measuredCharacters(line)).toBe(expected);
  });

  test.prop({ line: fc.string({ maxLength: 20, unit: "grapheme-ascii" }) })(
    "keeps a line of printable ASCII as typed",
    ({ line }) => {
      expect(measuredCharacters(line)).toBe(line);
    },
  );
});
