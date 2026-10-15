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
 * The most lines a paragraph's greedy wrap may give for it to be balanced: CSS lets a user agent
 * "treat this value as auto if there are more than ten lines to balance" (`REF-CSS-TEXT-4`, §5.4).
 */
const MAXIMUM_BALANCED_LINES = 10;

/**
 * The most tokens a paragraph may hold for it to be balanced, so that the candidate lines stay
 * few; CSS leaves the algorithm to the user agent (`REF-CSS-TEXT-4`, §5.4; provisional, VAL-015).
 */
const MAXIMUM_BALANCED_TOKENS = 500;

/**
 * Whether a paragraph is small enough to be balanced: its greedy wrap gives from 2 to 10 lines,
 * and it holds at most 500 tokens (`DERIV-text-box`, step 9).
 *
 * @kind domain
 * @param lineCount - the number of lines of its greedy wrap
 * @param tokenCount - the number of its tokens
 * @returns `true` when it may be balanced
 * @see DERIV-text-box
 */
function isWithinBalanceBounds(lineCount: number, tokenCount: number): boolean {
  return (
    lineCount >= 2 && lineCount <= MAXIMUM_BALANCED_LINES && tokenCount <= MAXIMUM_BALANCED_TOKENS
  );
}

/**
 * The lines of a paragraph wrapped in a balanced way: as many as the greedy wrap gives, cut where
 * the sum of the squares of the room each line leaves is the least (`leastRaggedBreaks`), on
 * equal cost at the later break, among the cuts whose lines grow while they fit
 * (`fittingLines`); the greedy cut is among them, so the balanced one is never more ragged. A
 * paragraph of one line or of more than 10, of more than 500 tokens (`isWithinBalanceBounds`), or
 * holding a word too long for a line of its own, keeps its greedy lines (`DERIV-text-box`,
 * step 9; `REF-CSS-TEXT-4`, §5.4, each paragraph processed separately).
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param paragraph - a paragraph of the text, in its case
 * @param text - its size, spacing and box
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

  const measures = isWithinBalanceBounds(greedy.length, tokens.length)
    ? tokens.map((token) => measuredToken(metrics, token))
    : [];

  if (
    measures.length === 0 ||
    measures.some(({ visible }) => !isMeasureFitting(metrics, visible, text))
  ) {
    return greedy;
  }

  const breaks = leastRaggedBreaks(fittingLines(metrics, measures, text), greedy.length);

  return linesFromStarts(tokens, [0, ...breaks, tokens.length]);
}
