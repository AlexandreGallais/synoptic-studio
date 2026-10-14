import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { characterUnitCount } from "./character-unit-count";

describe("characterUnitCount (REF-CSS-TEXT-3)", () => {
  it.each([
    { expected: 11, line: "Hello World" },
    { expected: 0, line: "" },
    // e and a combining acute (Mn): one unit.
    { expected: 1, line: "e\u{301}" },
    { expected: 2, line: "q\u{301}q" },
    // Hebrew shin, qamats (U+05B8, Mn), lamed: two units.
    { expected: 2, line: "\u{5E9}\u{5B8}\u{5DC}" },
    // Marks opening the line make one unit.
    { expected: 2, line: "\u{301}\u{302}a" },
    // A character beyond the basic plane is one unit.
    { expected: 2, line: "a\u{1F600}" },
  ])("counts $expected units in '$line'", ({ expected, line }) => {
    expect(characterUnitCount(line)).toBe(expected);
  });

  test.prop({ line: fc.string({ maxLength: 20, unit: "grapheme-ascii" }) })(
    "counts one unit per character in a line without marks",
    ({ line }) => {
      expect(characterUnitCount(line)).toBe(line.length);
    },
  );
});
