import { characterUnits } from "../text-block";

import { withoutTrailingSpaces } from "./without-trailing-spaces";

import type { LineWrap } from "./line-wrap";

/** The space U+0020, which never opens a line: the spaces after a word stay with it. */
const SPACE = " ";

/**
 * A word too long for its box broken at a character, as `overflow-wrap: anywhere`: on the line
 * being filled, each of its typographic character units (`characterUnits`) joins the line when
 * the line with it fits, and otherwise ends the line and starts the next, without a hyphen; a line
 * always takes at least one unit other than the spaces that open it, and the spaces after the word
 * stay with its last part (`DERIV-text-box`, step 4). Each line so takes the longest run of units
 * that fits, unless a negative spacing narrows a longer run again: the cut comes at the first unit
 * that does not fit.
 *
 * @kind domain
 * @param wrap - the paragraph wrapped so far, its line being filled
 * @param word - the word, with the spaces after it
 * @param isFitting - whether a line fits the box (`isLineFitting`)
 * @returns the paragraph wrapped with the word, its last part on the line being filled
 * @see DERIV-text-box
 */
export function wordBrokenAtCharacters(
  wrap: LineWrap,
  word: string,
  isFitting: (line: string) => boolean,
): LineWrap {
  return characterUnits(word).reduce<LineWrap>(
    (state, unit) =>
      unit === SPACE || withoutTrailingSpaces(state.line) === "" || isFitting(state.line + unit)
        ? { line: state.line + unit, lines: state.lines }
        : { line: unit, lines: [...state.lines, state.line] },
    wrap,
  );
}
