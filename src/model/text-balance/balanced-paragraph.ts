import {
  isMeasureFitting,
  measuredToken,
  paragraphTokens,
  withoutTrailingSpaces,
  wrappedParagraph,
} from "../text-wrap";

import { fittingLines } from "./fitting-lines";
import { leastRaggedBreaks } from "./least-ragged-breaks";

import type { FontMetrics } from "../font";
import type { Text } from "../text";

/**
 * The balanced lines of a paragraph from its tokens and the first token of each line: a line
 * ended by a wrap loses its trailing spaces, the last keeps those typed (`DERIV-text-box`,
 * step 2).
 *
 * @kind domain
 * @param tokens - the paragraph's tokens
 * @param starts - the first token of each line, then the number of tokens
 * @returns the lines, in order
 * @see DERIV-text-box
 */
function linesFromStarts(tokens: readonly string[], starts: readonly number[]): readonly string[] {
  return starts.slice(1).map((end, index) => {
    const line = tokens.slice(starts[index], end).join("");

    return end === tokens.length ? line : withoutTrailingSpaces(line);
  });
}

/**
 * The lines of a paragraph wrapped in a balanced way: as many as the greedy wrap gives, cut where
 * the sum of the squares of the room each line leaves is the least (`leastRaggedBreaks`), on
 * equal cost at the later break; the greedy cut is among the candidates, so the balanced one is
 * never more ragged. A paragraph of one line, or holding a word too long for a line of its own,
 * keeps its greedy lines (`DERIV-text-box`, step 9; `REF-CSS-TEXT-4`, §5.4, each paragraph
 * processed separately).
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param paragraph - a paragraph of the text, in its case
 * @param text - its size, spacing, box and long-word option
 * @returns its lines, in order; one empty line for an empty paragraph
 * @see DERIV-text-box
 */
export function balancedParagraph(
  metrics: FontMetrics,
  paragraph: string,
  text: Text,
): readonly string[] {
  const greedy = wrappedParagraph(metrics, paragraph, text);
  const tokens = paragraphTokens(paragraph);
  const measures = tokens.map((token) => measuredToken(metrics, token));

  if (
    greedy.length < 2 ||
    measures.some(({ visible }) => !isMeasureFitting(metrics, visible, text))
  ) {
    return greedy;
  }

  const breaks = leastRaggedBreaks(fittingLines(metrics, measures, text), greedy.length);

  return linesFromStarts(tokens, [0, ...breaks, tokens.length]);
}
