import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_UNKERNED, HELLO_TEXT } from "../../../tests";
import { EPSILON } from "../../math";
import { TEXT_OPTIONS } from "../text";
import { textBlock } from "../text-block";

import { positionedLines } from "./positioned-lines";

import type { Text } from "../text";
import type { PositionedLine } from "./positioned-line";

/**
 * The lines of a text in Arial 16 without kerning, placed in its measured block.
 *
 * @param text - its content, alignment, direction and position
 * @returns its lines
 */
function placed(text: Text): readonly PositionedLine[] {
  return positionedLines(ARIAL_UNKERNED, text, textBlock(ARIAL_UNKERNED, text).width);
}

/** "Hello" over "World" in Arial 16, without kerning. */
const TWO_LINES = { ...HELLO_TEXT, content: "Hello\nWorld" };

describe("positionedLines (DERIV-text-width steps 4 and 5)", () => {
  it.each([
    // Widths 4667 × 16 / 2048 = 36.4609375 and 5348 × 16 / 2048 = 41.78125: room 5.3203125.
    { alignment: "left", hello: 0 },
    { alignment: "center", hello: 2.66015625 },
    { alignment: "right", hello: 5.3203125 },
  ] as const)(
    "[F05.AC4] places 'Hello' at $hello and 'World' at 0, aligned $alignment",
    ({ alignment, hello }) => {
      const lines = placed({ ...TWO_LINES, alignment });

      // Baselines 1854 × 16 / 2048 = 14.484375, then 14.484375 + 18.3984375 = 32.8828125.
      expect(lines.map((line) => [line.x, line.y, line.width])).toEqual([
        [hello, 14.484375, 36.4609375],
        [0, 32.8828125, 41.78125],
      ]);
    },
  );

  it("[F05.AC4] moves every line with the block's top-left corner", () => {
    const [first] = placed({
      ...TWO_LINES,
      alignment: "center",
      x: 10,
      y: 20,
    });

    // 10 + 2.66015625 and 20 + 14.484375.
    expect(first).toEqual({
      content: "Hello",
      left: 12.66015625,
      width: 36.4609375,
      x: 12.66015625,
      y: 34.484375,
    });
  });

  it("[F05.AC4] writes a right-to-left line at its right edge, aligned as typed", () => {
    const [first, second] = placed({ ...TWO_LINES, direction: "rtl" });

    // Aligned left: 'Hello' from 0 to 36.4609375, 'World' from 0 to 41.78125.
    expect([first?.left, first?.x, second?.x]).toEqual([0, 36.4609375, 41.78125]);
  });

  it("[F05.AC4] writes the lines in the text's case", () => {
    const lines = placed({ ...TWO_LINES, textCase: "upper" });

    expect(lines.map((line) => line.content)).toEqual(["HELLO", "WORLD"]);
  });

  it("keeps an empty line in its place, a line height taken", () => {
    const lines = placed({ ...HELLO_TEXT, content: "Hello\n\nWorld" });

    // Third baseline: 14.484375 + 2 × 18.3984375 = 51.28125.
    expect(lines.map((line) => line.content)).toEqual(["Hello", "", "World"]);
    expect(lines[2]?.y).toBe(51.28125);
  });

  it("[F05.AC4] moves a line tightened past nothing right of the block when aligned right", () => {
    // "HH": 2 × 1479 × 16 / 2048 = 23.109375, letter spacing −30 once: −6.890625; block 0 wide.
    const [line] = placed({ ...HELLO_TEXT, alignment: "right", content: "HH", letterSpacing: -30 });

    expect([line?.width, line?.left]).toEqual([-6.890625, 6.890625]);
  });

  it("gives an empty text no line", () => {
    expect(placed({ ...HELLO_TEXT, content: "" })).toEqual([]);
  });

  const words = fc.stringMatching(/^[HWelord ]{0,8}(?:\n[HWelord ]{0,8}){0,3}$/u);

  test.prop({
    alignment: fc.constantFrom(...TEXT_OPTIONS.alignments),
    content: words,
    x: fc.integer({ max: 1000, min: -1000 }),
  })("keeps every line inside the measured block", ({ alignment, content, x }) => {
    const text = { ...HELLO_TEXT, alignment, content, x };
    const { width } = textBlock(ARIAL_UNKERNED, text);

    for (const line of placed(text)) {
      expect(line.left).toBeGreaterThanOrEqual(x - EPSILON);
      expect(line.left + line.width).toBeLessThanOrEqual(x + width + EPSILON);
    }
  });
});
