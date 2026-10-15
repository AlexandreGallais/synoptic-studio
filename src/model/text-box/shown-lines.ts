import { isLineFitting, withoutTrailingSpaces, wrappedLines } from "../text-wrap";

import { ellipsisCut } from "./ellipsis-cut";
import { shownLineCount } from "./shown-line-count";
import { textBlockLineHeights } from "./text-block-line-heights";
import { textEllipsis } from "./text-ellipsis";

import type { FontMetrics } from "../font";
import type { Text } from "../text";
import type { TextInBox } from "../text-wrap";

/**
 * A line shown, ended with an ellipsis when it is the last of a text cut, or when it is wider than
 * the box and its cut removes at least one unit: the first character of a line is "clipped rather
 * than ellipsed", so a line of one character wider than the box stays as it is
 * (`DERIV-text-box`, step 6).
 *
 * @kind domain
 * @param context - the text and the metrics of its face
 * @param line - a line shown, in the text's case
 * @param isLastOfCut - whether it is the last line shown of a text that lines were left out of
 * @returns the line, with its ellipsis when it gets one
 * @see REF-CSS-OVERFLOW-3
 */
function shownLine(context: TextInBox, line: string, isLastOfCut: boolean): string {
  if (!isLastOfCut && isLineFitting(context.metrics, line, context.text)) {
    return line;
  }

  const kept = ellipsisCut(context, line);

  return !isLastOfCut && kept === withoutTrailingSpaces(line)
    ? line
    : kept + textEllipsis(context.metrics);
}

/**
 * The lines a text shows: a text that fits its content shows all its lines (F05); in a box, the
 * lines it wraps into (`wrappedLines`), as many as its box and its maximum show
 * (`shownLineCount`), and, with the ellipsis on, the last line shown ends with an ellipsis when
 * lines other than empty ones were left out, as does a line wider than the box when its cut
 * removes a unit (`ellipsisCut`); with it off, a text cut stops at its last line shown and a word
 * too long is drawn whole (`DERIV-text-box`, steps 5 and 6).
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param text - its content, case, size, spacing, box and options
 * @returns its lines shown, in order; none for an empty content
 * @see DERIV-text-box
 */
export function shownLines(metrics: FontMetrics, text: Text): readonly string[] {
  const wrapped = wrappedLines(metrics, text);

  if (!text.isInBox) {
    return wrapped;
  }

  const count = shownLineCount(text, textBlockLineHeights(metrics, text), wrapped.length);
  const isCut = wrapped.slice(count).some((line) => line !== "");
  const shown = wrapped.slice(0, count);

  return text.hasEllipsis
    ? shown.map((line, index) => shownLine({ metrics, text }, line, isCut && index === count - 1))
    : shown;
}
