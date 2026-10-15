import { describe, expect, it } from "vitest";

import { ARIAL_BOX_METRICS, HELLO_TEXT } from "../../../tests";

import { isLineFitting } from "./is-line-fitting";
import { wordBrokenAtCharacters } from "./word-broken-at-characters";

/** Arial 16 without kerning in a box 50 wide. */
const BOXED = { ...HELLO_TEXT, boxWidth: 50, isInBox: true };

/**
 * Whether a line fits a box of a given width, in Arial 16 without kerning.
 *
 * @param boxWidth - in user units, an integer
 * @returns a test of a line against that box
 */
function fitsIn(boxWidth: number): (line: string) => boolean {
  return (line) => isLineFitting(ARIAL_BOX_METRICS, line, { ...BOXED, boxWidth });
}

describe("wordBrokenAtCharacters (DERIV-text-box step 4)", () => {
  it("[F14.AC1] breaks 'running' after 'runnin', the longest run that fits a box 50 wide", () => {
    // "runnin" 5693 / 128 = 44.47656 ≤ 50; with g, 6832 / 128 = 53.375 > 50. No hyphen.
    expect(wordBrokenAtCharacters({ line: "", lines: ["is"] }, "running", fitsIn(50))).toEqual({
      line: "g",
      lines: ["is", "runnin"],
    });
  });

  it("breaks a word several times, its last part on the line being filled with its spaces", () => {
    // A box 12 wide: "He" 2618 / 128 = 20.45 > 12, so each of "H" (11.55) and "e" (8.90) goes
    // alone; "ll" 910 / 128 = 7.10938 fits; "llo" 15.99 does not; "o" stays with its spaces.
    expect(wordBrokenAtCharacters({ line: "", lines: [] }, "Hello  ", fitsIn(12))).toEqual({
      line: "o  ",
      lines: ["H", "e", "ll"],
    });
  });

  it("keeps at least one character on a line, even wider than the box", () => {
    // A box 0 wide: each character alone, "H" 11.55 overflowing.
    expect(wordBrokenAtCharacters({ line: "", lines: [] }, "Hel", fitsIn(0))).toEqual({
      line: "l",
      lines: ["H", "e"],
    });
  });

  it("never counts the spaces opening a line as the character kept", () => {
    // "  H" 2 × 569 + 1479 = 2617 units, 20.44531 > 10: still on the line of its spaces.
    expect(wordBrokenAtCharacters({ line: "", lines: [] }, "  He", fitsIn(10))).toEqual({
      line: "e",
      lines: ["  H"],
    });
  });

  it("keeps a nonspacing mark with its character", () => {
    // "e" and a combining acute make one unit, never cut between them. "H" 11.55 fits 12; "Hé",
    // composed into U+00E9 which the fixture lacks (glyph 0, 1536): 3015 / 128 = 23.55 does not.
    expect(wordBrokenAtCharacters({ line: "", lines: [] }, "He\u{301}", fitsIn(12))).toEqual({
      line: "e\u{301}",
      lines: ["H"],
    });
  });
});
