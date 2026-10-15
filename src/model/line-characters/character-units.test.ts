import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { characterUnits } from "./character-units";

describe("characterUnits (REF-CSS-TEXT-3)", () => {
  it.each([
    { expected: ["r", "u", "n"], line: "run" },
    { expected: [], line: "" },
    // e and a combining acute (Mn) stay together.
    { expected: ["e\u{301}", "q"], line: "e\u{301}q" },
    // Hebrew shin and qamats (U+05B8, Mn), then lamed.
    { expected: ["\u{5E9}\u{5B8}", "\u{5DC}"], line: "\u{5E9}\u{5B8}\u{5DC}" },
    // Marks opening the line make one unit.
    { expected: ["\u{301}\u{302}", "a"], line: "\u{301}\u{302}a" },
    // A character beyond the basic plane is one unit, not two halves.
    { expected: ["a", "\u{1F600}"], line: "a\u{1F600}" },
  ])("cuts '$line' into $expected", ({ expected, line }) => {
    expect(characterUnits(line)).toEqual(expected);
  });

  test.prop({ line: fc.string({ maxLength: 20, unit: "binary" }) })(
    "gives back the line joined",
    ({ line }) => {
      expect(characterUnits(line).join("")).toBe(line);
    },
  );
});
