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
});
