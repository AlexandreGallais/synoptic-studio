import { joinedLineMeasure } from "../text-block";

import { isMeasureFitting } from "./is-measure-fitting";
import { measuredToken } from "./measured-token";
import { withoutTrailingSpaces } from "./without-trailing-spaces";

import type { LineWrap } from "./line-wrap";
import type { TextInBox } from "./text-in-box";

/**
 * The line a wrap ends: the line being filled without its trailing spaces, unless it is still
 * empty.
 *
 * @kind domain
 * @param wrap - the step before, its line being filled
 * @returns the line ended, or none
 * @see DERIV-text-box
 */
function endedLine(wrap: LineWrap): readonly string[] {
  const { text } = wrap.line;

  return text === "" ? [] : [withoutTrailingSpaces(text)];
}

/**
 * A paragraph wrapped with one more token: the token joins the line being filled when the line
 * with it, its trailing spaces left out, fits; otherwise it starts the next line, where a word too
 * long for a line of its own stays whole, overflowing (`DERIV-text-box`, steps 2 to 4). The
 * line's measure grows by the token's, measured once (`joinedLineMeasure`).
 *
 * @kind domain
 * @param wrap - the step before, its line being filled
 * @param token - the next token
 * @param context - the text and its face's metrics
 * @returns the lines the token ends, and the line being filled after it
 * @see DERIV-text-box
 */
export function lineWrapWithToken(wrap: LineWrap, token: string, context: TextInBox): LineWrap {
  const { metrics, text } = context;
  const { visible, whole } = measuredToken(metrics, token);

  return wrap.line.text !== "" &&
    isMeasureFitting(metrics, joinedLineMeasure(metrics, wrap.line, visible), text)
    ? { ended: [], line: joinedLineMeasure(metrics, wrap.line, whole) }
    : { ended: endedLine(wrap), line: whole };
}
