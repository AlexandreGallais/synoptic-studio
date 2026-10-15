import { describe, expect, it } from "vitest";

import { ARIAL_BOX_METRICS, HELLO_TEXT } from "../../../tests";
import { measuredLine } from "../text-block";

import { wordBrokenAtCharacters } from "./word-broken-at-characters";

import type { LineWrap } from "./line-wrap";

/**
 * A word broken in a box of Arial 16 without kerning, from an empty line, as lines.
 *
 * @param word - the word, with the spaces after it
 * @param boxWidth - in user units, an integer
 * @param lines - the lines already ended
 * @returns the lines ended, then the line being filled
 */
function brokenLines(word: string, boxWidth: number, lines: readonly string[]): readonly string[] {
  const empty: LineWrap = { line: measuredLine(ARIAL_BOX_METRICS, ""), lines };
  const text = { ...HELLO_TEXT, boxWidth, isInBox: true };
  const wrap = wordBrokenAtCharacters(empty, word, { metrics: ARIAL_BOX_METRICS, text });

  return [...wrap.lines, wrap.line.text];
}

describe("wordBrokenAtCharacters (DERIV-text-box step 4)", () => {
  it("[F14.AC1] breaks 'running' after 'runnin', the longest run that fits a box 50 wide", () => {
    // "runnin" 5693 / 128 = 44.47656 ≤ 50; with g, 6832 / 128 = 53.375 > 50. No hyphen.
    expect(brokenLines("running", 50, ["is"])).toEqual(["is", "runnin", "g"]);
  });

  it("breaks a word several times, its last part on the line being filled with its spaces", () => {
    // A box 12 wide: "He" 2618 / 128 = 20.45 > 12 and "el" 1594 / 128 = 12.45 > 12, so "H" and
    // "e" go alone; "ll" 910 / 128 = 7.10938 fits; "llo" 2049 / 128 = 16.00781 does not; "o"
    // keeps the spaces after the word.
    expect(brokenLines("Hello  ", 12, [])).toEqual(["H", "e", "ll", "o  "]);
  });

  it("keeps at least one character on a line, even wider than the box", () => {
    // A box 0 wide: each character alone, "H" 11.55 overflowing.
    expect(brokenLines("Hel", 0, [])).toEqual(["H", "e", "l"]);
  });

  it("never counts the spaces opening a line as the character kept", () => {
    // "  H" 2 × 569 + 1479 = 2617 units, 20.44531 > 10: still on the line of its spaces.
    expect(brokenLines("  He", 10, [])).toEqual(["  H", "e"]);
  });

  it("keeps a nonspacing mark with its character", () => {
    // "e" and a combining acute make one unit, never cut between them. "H" 11.55 fits 12; "Hé",
    // composed into U+00E9 which the fixture lacks (glyph 0, 1536): 3015 / 128 = 23.55 does not.
    expect(brokenLines("He\u{301}", 12, [])).toEqual(["H", "e\u{301}"]);
  });

  it("never ends a line with a no-break space: it moves with the unit after it", () => {
    // "Hello" 36.46 ≤ 50; "Hello" + no-break space (glyph 0, 1536) = 6203 / 128 = 48.46 would fit,
    // but the no-break space goes with "W": 8136 / 128 = 63.56 > 50, so the cut comes before it.
    // Then "\u{A0}Worl" (1536 + 1933 + 1139 + 682 + 455) / 128 = 44.88 ≤ 50, with d 53.78 > 50.
    const lines = brokenLines("Hello\u{A0}World", 50, []);

    expect(lines).toEqual(["Hello", "\u{A0}Worl", "d"]);
    expect(lines.some((line) => line.endsWith("\u{A0}"))).toBe(false);
  });

  it("keeps a no-break space and its unit as the one unit a line takes", () => {
    // A box 0 wide: the no-break space and "r" are one unit, kept together on one line.
    expect(brokenLines("\u{A0}r", 0, [])).toEqual(["\u{A0}r"]);
  });

  it("places a no-break space ending the word as a unit of its own, measured for fit", () => {
    // A box 0 wide: no unit follows the no-break space, which goes alone, the cut before it.
    expect(brokenLines("H\u{A0}", 0, [])).toEqual(["H", "\u{A0}"]);
    expect(brokenLines("H\u{A0} ", 0, [])).toEqual(["H", "\u{A0} "]);
  });
});
