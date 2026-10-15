import { describe, expect, it } from "vitest";

import { withoutTrailingSpaces } from "./without-trailing-spaces";

describe("withoutTrailingSpaces (REF-UAX14, SP)", () => {
  it.each([
    { expected: "Hello", line: "Hello  " },
    { expected: "Hello", line: "Hello" },
    // Only the spaces at the end go: those at the start and inside stay.
    { expected: "  Hello World", line: "  Hello World " },
    // A no-break space and a tab are not the space U+0020.
    { expected: "Hello\u{A0}", line: "Hello\u{A0}" },
    { expected: "Hello\t", line: "Hello\t " },
    { expected: "", line: " ".repeat(3) },
    { expected: "", line: "" },
  ])("gives '$expected' for '$line'", ({ expected, line }) => {
    expect(withoutTrailingSpaces(line)).toBe(expected);
  });

  it("reads 40 000 spaces once, in well under a second", { timeout: 500 }, () => {
    // A pattern / +$/ would try each of the 40 000 first spaces as a start: 8 × 10⁸ steps.
    const line = `${" ".repeat(40_000)}x${" ".repeat(40_000)}`;

    expect(withoutTrailingSpaces(line)).toBe(`${" ".repeat(40_000)}x`);
  });
});
