import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { paragraphTokens } from "./paragraph-tokens";

describe("paragraphTokens (DERIV-text-box step 1)", () => {
  it.each([
    { expected: ["Hello ", "World"], paragraph: "Hello World" },
    // A run of spaces stays with the word before it: the break comes after its last space (LB18).
    { expected: ["Hello  ", "World"], paragraph: "Hello  World" },
    // A no-break space opens no break (LB12): one token.
    { expected: ["Hello\u{A0}World"], paragraph: "Hello\u{A0}World" },
    // A hyphen opens no break either, in F14's scope.
    { expected: ["Pump ", "P-101 ", "is ", "running"], paragraph: "Pump P-101 is running" },
    // The spaces opening the paragraph join its first token.
    { expected: ["  Hello ", "World "], paragraph: "  Hello World " },
    { expected: [" ".repeat(3)], paragraph: " ".repeat(3) },
    { expected: [], paragraph: "" },
  ])("cuts '$paragraph' into $expected", ({ expected, paragraph }) => {
    expect(paragraphTokens(paragraph)).toEqual(expected);
  });

  test.prop({ paragraph: fc.stringMatching(/^[ a\u{A0}]{0,30}$/u) })(
    "gives back the paragraph joined, each token a run of other characters and its spaces",
    ({ paragraph }) => {
      const tokens = paragraphTokens(paragraph);

      expect(tokens.join("")).toBe(paragraph);
      expect(tokens.slice(1).every((token) => /^[^ ]+ *$/u.test(token))).toBe(true);
    },
  );
});
