import { describe, expect, it } from "vitest";

import { ARIAL_METRICS, HELLO_TEXT } from "../../tests";

import { readShownTextBlock } from "./read-shown-text-block";

/** A block "Hello World" could measure, the numbers only standing for a measured block. */
const MEASURED = { height: 19.359375, width: 85.5 };

/** The block of an empty text: no line, no area. */
const EMPTY_BLOCK = { height: 0, width: 0 };

describe("readShownTextBlock (US-075)", () => {
  it("[F40.AC2] keeps the measured block of a text with a content", () => {
    expect(readShownTextBlock(ARIAL_METRICS, HELLO_TEXT, MEASURED)).toEqual(MEASURED);
  });

  it("[F40.AC2] shows an empty text fitting its content 1 wide and one automatic line tall", () => {
    const text = { ...HELLO_TEXT, content: "" };

    // Arial at 16: (1854 + 434 + 67) × 16 / 2048 = 2355 × 16 / 2048 = 18.3984375.
    expect(readShownTextBlock(ARIAL_METRICS, text, EMPTY_BLOCK)).toEqual({
      height: 18.3984375,
      width: 1,
    });
  });

  it("[F40.AC2] shows an empty text one line tall at the line height chosen", () => {
    const text = { ...HELLO_TEXT, content: "", lineHeight: 24 };

    expect(readShownTextBlock(ARIAL_METRICS, text, EMPTY_BLOCK)).toEqual({ height: 24, width: 1 });
  });

  it("[F40.AC2] keeps the block of an empty text in a box, its frame being the box", () => {
    const text = { ...HELLO_TEXT, boxHeight: 40, boxWidth: 60, content: "", isInBox: true };

    expect(readShownTextBlock(ARIAL_METRICS, text, EMPTY_BLOCK)).toEqual(EMPTY_BLOCK);
  });

  it("[F40.AC2] counts a content of a space alone as a content, its block kept", () => {
    const text = { ...HELLO_TEXT, content: " " };
    // A space at 16: 569 × 16 / 2048 = 4.4453125 wide, a line tall.
    const block = { height: 18.3984375, width: 4.4453125 };

    expect(readShownTextBlock(ARIAL_METRICS, text, block)).toEqual(block);
  });
});
