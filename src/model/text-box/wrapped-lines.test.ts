import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_BOX_METRICS, HELLO_TEXT } from "../../../tests";
import { textLines } from "../text-block";

import { wrappedLines } from "./wrapped-lines";

import type { Text } from "../text";

/** Arial 16 without kerning in a box 50 wide. */
const BOXED: Text = { ...HELLO_TEXT, boxHeight: 1000, boxWidth: 50, isInBox: true };

describe("wrappedLines (DERIV-text-box step 1)", () => {
  it("[F14.AC1] wraps each paragraph on its own", () => {
    // "Hello World" wraps into "Hello" / "World"; "Hello" after the line break stays alone.
    expect(wrappedLines(ARIAL_BOX_METRICS, { ...BOXED, content: "Hello World\nHello" })).toEqual([
      "Hello",
      "World",
      "Hello",
    ]);
  });

  it("wraps the content in its case, and gives no line for an empty content", () => {
    // In sentence case "Hello world": 4667 + 569 + w 1536 (glyph 0) + 1139 + 682 + 455 + 1139
    // = 10 187 units, 79.58594 > 50, so it wraps after the space.
    const text = { ...BOXED, content: "hello world", textCase: "sentence" } as const;

    expect(wrappedLines(ARIAL_BOX_METRICS, text)).toEqual(["Hello", "world"]);
    expect(wrappedLines(ARIAL_BOX_METRICS, { ...BOXED, content: "" })).toEqual([]);
  });

  it("[F14.AC6] keeps the lines of F05 for a text that fits its content", () => {
    // "Hello World" in a box 50 wide would wrap; fitting its content it stays one line.
    const text = { ...BOXED, content: "Hello World\nHello", isInBox: false };

    expect(wrappedLines(ARIAL_BOX_METRICS, text)).toEqual(["Hello World", "Hello"]);
  });

  test.prop({ content: fc.stringMatching(/^[ \nHWdelor]{0,40}$/u) })(
    "[F14.AC6] gives a text that fits its content the lines of its content",
    ({ content }) => {
      const text = { ...BOXED, content, isInBox: false };

      expect(wrappedLines(ARIAL_BOX_METRICS, text)).toEqual(textLines(content));
    },
  );
});
