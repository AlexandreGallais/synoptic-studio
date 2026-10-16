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
 * The line of a text cut that ends with its ellipsis: none when only empty lines were left out;
 * else the last line shown holding text, so that "…" never stands alone on an empty line, or the
 * last line shown when none holds text. A line of spaces is empty, its spaces trimmed as at the
 * end of every line (`DERIV-text-box`, steps 2 and 6, provisional).
 *
 * @kind domain
 * @param shown - the lines the box shows
 * @param leftOut - the lines left out after them
 * @returns the index of the line that ends with the ellipsis, undefined when the text is not cut
 * @see DERIV-text-box
 */
function lastOfCut(shown: readonly string[], leftOut: readonly string[]): number | undefined {
  if (leftOut.every((line) => line === "")) {
    return undefined;
  }

  const lastWithText = shown.findLastIndex((line) => line !== "");

  return lastWithText === -1 ? shown.length - 1 : lastWithText;
}

/**
 * The lines a box shows with the ellipsis on: the line of a text cut that takes the ellipsis
 * (`lastOfCut`) ends with it, the empty lines shown after it dropped, and every other line wider
 * than the box ends with one when its cut removes a unit (`shownLine`).
 *
 * @kind domain
 * @param context - the text and the metrics of its face
 * @param shown - the lines the box shows
 * @param leftOut - the lines left out after them
 * @returns the lines shown, with their ellipses
 * @see DERIV-text-box
 */
function ellipsedLines(
  context: TextInBox,
  shown: readonly string[],
  leftOut: readonly string[],
): readonly string[] {
  const last = lastOfCut(shown, leftOut);

  return shown
    .slice(0, last === undefined ? shown.length : last + 1)
    .map((line, index) => shownLine(context, line, index === last));
}

/**
 * The lines a text shows: a text that fits its content shows all its lines (F05); in a box, the
 * lines it wraps into (`wrappedLines`), as many as its box and its maximum show
 * (`shownLineCount`), and, with the ellipsis on, when lines other than empty ones were left out,
 * the last line shown holding text ends with an ellipsis and the empty lines shown after it are
 * dropped (`lastOfCut`), as a line wider than the box ends with one when its cut removes a unit
 * (`ellipsisCut`); with it off, a text cut stops at its last line shown and a word
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
  const shown = wrapped.slice(0, count);

  return text.hasEllipsis ? ellipsedLines({ metrics, text }, shown, wrapped.slice(count)) : shown;
}
