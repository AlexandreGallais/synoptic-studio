import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { capitalized } from "./capitalized";

describe("capitalized (Q36)", () => {
  it.each([
    { expected: "Pump", word: "pump" },
    // The rest is kept as typed.
    { expected: "PUMP", word: "pUMP" },
    { expected: "101", word: "101" },
    { expected: "", word: "" },
    // A whole code point: U+10428 has the capital U+10400.
    { expected: "\u{10400}", word: "\u{10428}" },
  ])("writes '$word' as '$expected'", ({ expected, word }) => {
    expect(capitalized(word)).toBe(expected);
  });

  test.prop({ word: fc.string({ maxLength: 20, unit: "grapheme-ascii" }) })(
    "changes only the first character, to its upper case",
    ({ word }) => {
      expect(capitalized(word)).toBe(word.charAt(0).toUpperCase() + word.slice(1));
    },
  );
});
