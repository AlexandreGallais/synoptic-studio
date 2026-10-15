import { ellipsizedLine } from "./ellipsized-line";
import { isLineFitting } from "./is-line-fitting";
import { shownLineCount } from "./shown-line-count";
import { textBlockLineHeights } from "./text-block-line-heights";
import { wrappedLines } from "./wrapped-lines";

import type { TextInBox } from "./text-in-box";
import type { FontMetrics } from "../font";
import type { Text } from "../text";

/**
 * The lines shown, each ended with an ellipsis when it is the last of a text cut or wider than the
 * box, its trailing spaces left out (`DERIV-text-box`, step 6).
 *
 * @kind domain
 * @param context - the text and the metrics of its face
 * @param shown - the lines shown, in order
 * @param isCut - whether lines were left out after them
 * @returns the lines with their ellipses
 * @see DERIV-text-box
 */
function withEllipses(
  context: TextInBox,
  shown: readonly string[],
  isCut: boolean,
): readonly string[] {
  const { metrics, text } = context;

  return shown.map((line, index) =>
    (isCut && index === shown.length - 1) || !isLineFitting(metrics, line, text)
      ? ellipsizedLine(context, line)
      : line,
  );
}

/**
 * The lines a text shows: a text that fits its content shows all its lines (F05); in a box, the
 * lines it wraps into (`wrappedLines`), as many as its box and its maximum show
 * (`shownLineCount`), and, with the ellipsis on, the last line shown ends with an ellipsis when
 * lines were left out, as does every line wider than the box (`ellipsizedLine`); with it off, a
 * text cut stops at its last line shown and a word moved whole is drawn whole
 * (`DERIV-text-box`, steps 5 and 6).
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

  return text.hasEllipsis ? withEllipses({ metrics, text }, shown, count < wrapped.length) : shown;
}
