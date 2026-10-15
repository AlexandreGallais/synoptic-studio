import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_BOX_METRICS, HELLO_TEXT } from "../../../tests";
import { lineWidth } from "../text-block";
import { isLineFitting, measuredToken, paragraphTokens, withoutTrailingSpaces } from "../text-wrap";

import { fittingLines } from "./fitting-lines";

import type { Text } from "../text";

/** Arial 16 without kerning in a box 110 wide. */
const BOXED: Text = {
  ...HELLO_TEXT,
  boxHeight: 1000,
  boxWidth: 110,
  isInBox: true,
  longWord: "move",
};

/**
 * The lines that fit a paragraph, as start and end tokens with their rooms.
 *
 * @param paragraph - a paragraph of Arial characters
 * @param text - the text it sits in
 * @returns for each end, the lines ending there
 */
function linesOf(paragraph: string, text: Text): ReturnType<typeof fittingLines> {
  const tokens = paragraphTokens(paragraph).map((token) => measuredToken(ARIAL_BOX_METRICS, token));

  return fittingLines(ARIAL_BOX_METRICS, tokens, text);
}

describe("fittingLines (DERIV-text-box steps 2, 3 and 9)", () => {
  it("[F14.AC3] lists the lines of 'Pump P-101 is running' that fit a box 110 wide", () => {
    // Pump 41.79688; "Pump P-101" 41.79688 + 4.44531 + 42.69531 = 88.9375; "Pump P-101 is"
    // 88.9375 + 4.44531 + 11.55469 = 104.9375; with "running" 104.9375 + 4.44531 + 53.375 > 110.
    // "P-101 is running" 58.69531 + 4.44531 + 53.375 = 116.51563 > 110; "is running" 69.375.
    const ending = linesOf("Pump P-101 is running", BOXED);
    const rooms = ending.map(({ lines }) => lines.map(({ room, start }) => [start, room]));

    expect(rooms).toEqual([
      [],
      [[0, 110 - 41.796875]],
      [
        [0, 110 - 88.9375],
        [1, 110 - 42.6953125],
      ],
      [
        [0, 110 - 104.9375],
        [1, 110 - 58.6953125],
        [2, 110 - 11.5546875],
      ],
      [
        [2, 110 - 69.375],
        [3, 110 - 53.375],
      ],
    ]);
  });

  it("closes a line at the first token that does not fit, as the greedy wrap", () => {
    // A box 50: "Hello World" 82.6875 > 50 closes the line from "Hello" before "World".
    expect(
      linesOf("Hello World", { ...BOXED, boxWidth: 50 }).map(({ lines }) => lines.length),
    ).toEqual([0, 1, 1]);
  });

  it("gives only the empty end for an empty paragraph", () => {
    expect(linesOf("", BOXED)).toEqual([{ lines: [] }]);
  });

  test.prop({
    boxWidth: fc.integer({ max: 120, min: 0 }),
    letterSpacing: fc.integer({ max: 5, min: -15 }),
    paragraph: fc.stringMatching(/^[ HWdelor\u{A0}]{0,30}$/u),
  })("[F14.AC3] gives each line the room its string leaves, and only lines that fit", (values) => {
    const text = { ...BOXED, boxWidth: values.boxWidth, letterSpacing: values.letterSpacing };
    const tokens = paragraphTokens(values.paragraph);

    for (const [end, { lines }] of linesOf(values.paragraph, text).entries()) {
      for (const { room, start } of lines) {
        const line = tokens.slice(start, end).join("");

        expect(isLineFitting(ARIAL_BOX_METRICS, line, text)).toBe(true);

        expect(room).toBeCloseTo(
          text.boxWidth - lineWidth(ARIAL_BOX_METRICS, withoutTrailingSpaces(line), text),
          9,
        );
      }
    }
  });
});
