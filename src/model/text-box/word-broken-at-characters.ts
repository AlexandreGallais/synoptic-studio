import { characterUnits } from "../line-characters";
import { joinedLineMeasure, measuredLine } from "../text-block";

import { isMeasureFitting } from "./is-measure-fitting";

import type { LineWrap } from "./line-wrap";
import type { TextInBox } from "./text-in-box";

/** The space U+0020, which never opens a line: the spaces after a word stay with it. */
const SPACE = " ";

/** The no-break space U+00A0, which never ends a line: it stays with the unit after it. */
const NO_BREAK_SPACE = "\u{A0}";

/** A word being broken: the paragraph wrapped so far, and the no-break spaces waiting for the unit after them. */
type WordWrap = {
  /** The paragraph wrapped so far. */
  readonly wrap: LineWrap;
  /** The no-break spaces read and not yet placed. */
  readonly pending: string;
};

/**
 * A paragraph wrapped with one more unit of a word too long: the unit joins the line being filled
 * when the line with it fits, when it is a space after the word, which is not measured for fit,
 * or when the line holds nothing but the spaces that open it; otherwise it starts the next line
 * (`DERIV-text-box`, step 4). Inside a word, spaces come only at its start and its end, so a line
 * ending with a space holds no other unit to come. A no-break space ending the word, measured for
 * fit, is a unit of its own.
 *
 * @kind domain
 * @param wrap - the paragraph wrapped so far
 * @param unit - the next unit, with the no-break spaces before it
 * @param context - the text and its face's metrics
 * @returns the paragraph wrapped with the unit
 * @see DERIV-text-box
 */
function wrapWithUnit(wrap: LineWrap, unit: string, context: TextInBox): LineWrap {
  const measure = measuredLine(context.metrics, unit);
  const joined = joinedLineMeasure(context.metrics, wrap.line, measure);

  return unit === SPACE ||
    wrap.line.text === "" ||
    wrap.line.text.endsWith(SPACE) ||
    isMeasureFitting(context.metrics, joined, context.text)
    ? { line: joined, lines: wrap.lines }
    : { line: measure, lines: [...wrap.lines, wrap.line.text] };
}

/**
 * A word too long for its box broken at a character, as `overflow-wrap: anywhere`: on the line
 * being filled, each of its typographic character units (`characterUnits`) joins the line when
 * the line with it fits, and otherwise ends the line and starts the next, without a hyphen; a line
 * always takes at least one unit other than the spaces that open it, the spaces after the word
 * stay with its last part, and a no-break space stays with the unit after it, so that no line
 * ends with one (`DERIV-text-box`, step 4). The cut comes at the first unit that does not fit:
 * the longest run that fits, unless a negative spacing narrows a longer run again. Each unit is
 * measured once and added to the line's measure (`joinedLineMeasure`).
 *
 * @kind domain
 * @param wrap - the paragraph wrapped so far, its line being filled
 * @param word - the word, with the spaces after it
 * @param context - the text and its face's metrics
 * @returns the paragraph wrapped with the word, its last part on the line being filled
 * @see DERIV-text-box
 */
export function wordBrokenAtCharacters(wrap: LineWrap, word: string, context: TextInBox): LineWrap {
  const end = characterUnits(word).reduce<WordWrap>(
    (state, unit) =>
      unit === NO_BREAK_SPACE
        ? { pending: state.pending + unit, wrap: state.wrap }
        : { pending: "", wrap: wrapWithUnit(state.wrap, state.pending + unit, context) },
    { pending: "", wrap },
  );

  return end.pending === "" ? end.wrap : wrapWithUnit(end.wrap, end.pending, context);
}
