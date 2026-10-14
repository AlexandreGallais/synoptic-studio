import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { textLines } from "./text-lines";

describe("textLines (F05)", () => {
  it.each([
    { content: "", expected: [] },
    { content: "Hello", expected: ["Hello"] },
    { content: "Hello\nWorld", expected: ["Hello", "World"] },
    // A content ending with a line feed ends with an empty line.
    { content: "Hello\n", expected: ["Hello", ""] },
    { content: "\n", expected: ["", ""] },
  ])("cuts $content into $expected", ({ content, expected }) => {
    expect(textLines(content)).toEqual(expected);
  });

  test.prop({ lines: fc.array(fc.stringMatching(/^[^\n]{0,10}$/u), { minLength: 1 }) })(
    "gives back the lines joined by line feeds, but for a single empty one",
    ({ lines }) => {
      const content = lines.join("\n");

      expect(textLines(content)).toEqual(content === "" ? [] : lines);
    },
  );
});
