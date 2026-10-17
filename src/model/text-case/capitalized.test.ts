import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { capitalized } from "./capitalized";

describe("capitalized (Q36)", () => {
  it.each([
    { expected: "Pump", word: "pump" },
    // The rest is kept as typed.
    { expected: "PUMP", word: "pUMP" },
    // No letter: nothing changes.
    { expected: "101", word: "101" },
    { expected: "", word: "" },
    // The first letter, past what is not a letter.
    { expected: " Pump", word: " pump" },
    { expected: "(Pump) on", word: "(pump) on" },
    { expected: "101Abc", word: "101abc" },
    // A whole code point: U+10428 has the capital U+10400.
    { expected: "\u{10400}", word: "\u{10428}" },
  ])("writes '$word' as '$expected'", ({ expected, word }) => {
    expect(capitalized(word)).toBe(expected);
  });

  test.prop({ word: fc.string({ maxLength: 20, unit: "grapheme-ascii" }) })(
    "changes only the first letter, to its upper case",
    ({ word }) => {
      const first = word.search(/[A-Za-z]/u);
      const expected =
        first === -1
          ? word
          : word.slice(0, first) + word.charAt(first).toUpperCase() + word.slice(first + 1);

      expect(capitalized(word)).toBe(expected);
    },
  );
});
