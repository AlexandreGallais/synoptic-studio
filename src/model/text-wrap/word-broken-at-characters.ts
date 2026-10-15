import { scan } from "../../math";
import { characterUnits, isNoBreakAfter } from "../line-characters";
import { joinedLineMeasure, measuredLine } from "../text-block";

import { isMeasureFitting } from "./is-measure-fitting";

import type { LineWrap } from "./line-wrap";
import type { TextInBox } from "./text-in-box";
import type { LineMeasure } from "../text-block";

/** The space U+0020, which never opens a line: the spaces after a word stay with it. */
const SPACE = " ";

/** A step of a word being broken: the lines it ends, the line being filled, the no-break spaces waiting. */
type WordWrap = {
  /** The lines this step ends. */
  readonly ended: readonly string[];
  /** The line being filled. */
  readonly line: LineMeasure;
  /** The no-break spaces read and not yet placed. */
  readonly pending: string;
};

/**
 * A line being filled with one more unit of a word too long: the unit joins the line when the
 * line with it fits, when it is a space after the word, which is not measured for fit, or when the
 * line holds nothing but the spaces that open it; otherwise the line ends and the unit starts the
 * next (`DERIV-text-box`, step 4). Inside a word, spaces come only at its start and its end, so a
 * line ending with a space holds no other unit to come. A no-break space ending the word, measured
 * for fit, is a unit of its own.
 *
 * @kind domain
 * @param line - the line being filled
 * @param unit - the next unit, with the no-break spaces before it
 * @param context - the text and its face's metrics
 * @returns the line it ends, if any, and the line being filled after it
 * @see DERIV-text-box
 */
function wrapWithUnit(line: LineMeasure, unit: string, context: TextInBox): LineWrap {
  const measure = measuredLine(context.metrics, unit);
  const joined = joinedLineMeasure(context.metrics, line, measure);

  return unit === SPACE ||
    line.text === "" ||
    line.text.endsWith(SPACE) ||
    isMeasureFitting(context.metrics, joined, context.text)
    ? { ended: [], line: joined }
    : { ended: [line.text], line: measure };
}

/**
 * A word being broken, one more unit read: a no-break space waits for the unit after it; any
 * other unit is placed with the no-break spaces before it (`wrapWithUnit`).
 *
 * @kind domain
 * @param state - the step before
 * @param unit - the next unit
 * @param context - the text and its face's metrics
 * @returns the step after it
 * @see DERIV-text-box
 */
function wordWrapWithUnit(state: WordWrap, unit: string, context: TextInBox): WordWrap {
  if (isNoBreakAfter(unit)) {
    return { ended: [], line: state.line, pending: state.pending + unit };
  }

  if (unit === SPACE && state.pending !== "") {
    const pending = wrapWithUnit(state.line, state.pending, context);

    return { ...pending, line: wrapWithUnit(pending.line, unit, context).line, pending: "" };
  }

  return { ...wrapWithUnit(state.line, state.pending + unit, context), pending: "" };
}

/**
 * A word too long for its box broken at a character, as `overflow-wrap: anywhere`: on the line
 * being filled, each of its typographic character units (`characterUnits`) joins the line when
 * the line with it fits, and otherwise ends the line and starts the next, without a hyphen; a line
 * always takes at least one unit other than the spaces that open it, the spaces after the word
 * stay with its last part, and a no-break space stays with the unit after it, so that no line
 * ends with one (`DERIV-text-box`, step 4). The cut comes at the first unit that does not fit:
 * the longest run that fits, unless a negative spacing narrows a longer run again. Each unit is
 * measured once and added to the line's measure (`joinedLineMeasure`), the units read by a scan.
 *
 * @kind domain
 * @param line - the line being filled
 * @param word - the word, with the spaces after it
 * @param context - the text and its face's metrics
 * @returns the lines the word ends, and the line being filled with its last part
 * @see DERIV-text-box
 */
export function wordBrokenAtCharacters(
  line: LineMeasure,
  word: string,
  context: TextInBox,
): LineWrap {
  const start: WordWrap = { ended: [], line, pending: "" };
  const steps = scan<WordWrap, string>(
    characterUnits(word),
    (state, unit) => wordWrapWithUnit(state, unit, context),
    start,
  );
  const end = steps.at(-1) ?? start;
  const last =
    end.pending === ""
      ? { ended: [], line: end.line }
      : wrapWithUnit(end.line, end.pending, context);

  return { ended: [...steps.flatMap(({ ended }) => ended), ...last.ended], line: last.line };
}
