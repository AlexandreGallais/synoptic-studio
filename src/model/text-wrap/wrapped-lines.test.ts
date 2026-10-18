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

  it("[F17.AC1] wraps each paragraph automatically, never balanced", () => {
    // Twice "Pump P-101 is running" in a box 110: "Pump P-101 is" (104.9375) / "running" each
    // time, not the even "Pump P-101" / "is running"; the paragraphs are never joined.
    const text = {
      ...BOXED,
      boxWidth: 110,
      content: "Pump P-101 is running\nPump P-101 is running",
    };

    expect(wrappedLines(ARIAL_BOX_METRICS, text)).toEqual([
      "Pump P-101 is",
      "running",
      "Pump P-101 is",
      "running",
    ]);
  });

  it("wraps the content in its case, and gives no line for an empty content", () => {
    // In title case "Hello World": 4667 + 569 + 1933 + 1139 + 682 + 455 + 1139 = 10 584 units,
    // 82.6875 > 50, so it wraps after the space; as typed it would give "hello", "world".
    const text = { ...BOXED, content: "hello world", textCase: "title" } as const;

    expect(wrappedLines(ARIAL_BOX_METRICS, text)).toEqual(["Hello", "World"]);

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
