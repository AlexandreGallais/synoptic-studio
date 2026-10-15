import { joinedLineMeasure, lineMeasureWidth } from "../text-block";
import { isMeasureFitting } from "../text-wrap";

import type { LinesEnding } from "./lines-ending";
import type { FontMetrics } from "../font";
import type { Text } from "../text";
import type { LineMeasure } from "../text-block";
import type { TextInBox, TokenMeasure } from "../text-wrap";

/** A line still open: the token it starts at and the measure of its tokens so far, spaces kept. */
type OpenLine = {
  /** The index of its first token. */
  readonly start: number;
  /** The measure of its tokens so far, with the spaces after the last. */
  readonly whole: LineMeasure;
};

/** A line open before a token, and the measure it would end with after that token. */
type ExtendedLine = {
  /** The line open before the token. */
  readonly line: OpenLine;
  /** The measure of the line ended after the token, its trailing spaces left out. */
  readonly visible: LineMeasure;
};

/** The paragraph read so far: the lines still open, and the lines ending before each token read. */
type LineScan = {
  /** The lines that fit with every token read, by their first token in order. */
  readonly open: readonly OpenLine[];
  /** For each token index read, and the end, the lines that fit and end right before it. */
  readonly ending: readonly LinesEnding[];
};

/**
 * The lines still open before a token, with a new one that starts at it, each with the measure it
 * would end with after the token (`joinedLineMeasure`, the token measured once).
 *
 * @kind domain
 * @param scan - the paragraph read so far
 * @param token - the next token's measures
 * @param metrics - the metrics of the face the text is drawn with
 * @returns the lines, by their first token in order
 * @see DERIV-text-box
 */
function extendedLines(
  scan: LineScan,
  token: TokenMeasure,
  metrics: FontMetrics,
): readonly ExtendedLine[] {
  const opening = {
    line: { start: scan.ending.length - 1, whole: token.whole },
    visible: token.visible,
  };

  return [
    ...scan.open.map((line) => ({
      line,
      visible: joinedLineMeasure(metrics, line.whole, token.visible),
    })),
    opening,
  ];
}

/**
 * The room a line leaves in its box: the box's width less the line's, its trailing spaces left out
 * (`DERIV-text-box`, step 9).
 *
 * Formula: W − w(trim(t)).
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param visible - the measure of the line without its trailing spaces
 * @param text - its size, spacing and box width
 * @returns the room left, in user units
 * @see DERIV-text-box
 */
function lineRoom(metrics: FontMetrics, visible: LineMeasure, text: Text): number {
  return text.boxWidth - lineMeasureWidth(metrics, visible, text);
}

/**
 * The paragraph read with one more token: of the lines open before it and the one starting at it,
 * those that fit with it end right after it and stay open, the token joined; the others close for
 * good, as the greedy wrap ends a line at the first token that does not fit (`DERIV-text-box`,
 * steps 3 and 9).
 *
 * @kind domain
 * @param scan - the paragraph read so far
 * @param token - the next token's measures
 * @param context - the text and its face's metrics
 * @returns the paragraph read with the token
 * @see DERIV-text-box
 */
function scanWithToken(scan: LineScan, token: TokenMeasure, context: TextInBox): LineScan {
  const { metrics, text } = context;
  const fitting = extendedLines(scan, token, metrics).filter((extended) =>
    isMeasureFitting(metrics, extended.visible, text),
  );
  const lines = fitting.map(({ line, visible }) => ({
    room: lineRoom(metrics, visible, text),
    start: line.start,
  }));
  const open = fitting.map(({ line }) =>
    line.start === scan.ending.length - 1
      ? line
      : { start: line.start, whole: joinedLineMeasure(metrics, line.whole, token.whole) },
  );

  return { ending: [...scan.ending, { lines }], open };
}

/**
 * The lines a paragraph may be cut into in its box, the candidates of the balanced wrap: for each
 * end, the lines of whole tokens that end there and fit, every shorter line from the same start
 * fitting too, so that the greedy wrap's lines are among them; each line leaves its room, the box's
 * width less its own, trailing spaces left out (`DERIV-text-box`, steps 2, 3 and 9). The lines are
 * measured by joining the tokens' measures (`joinedLineMeasure`), each token measured once, at a
 * cost of the number of tokens times the most tokens a line holds.
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param tokens - the paragraph's tokens, measured
 * @param text - its size, spacing and box width
 * @returns for each index from 0 to the number of tokens, the lines ending right before it
 * @see DERIV-text-box
 */
export function fittingLines(
  metrics: FontMetrics,
  tokens: readonly TokenMeasure[],
  text: Text,
): readonly LinesEnding[] {
  const context = { metrics, text };

  return tokens.reduce<LineScan>((scan, token) => scanWithToken(scan, token, context), {
    ending: [{ lines: [] }],
    open: [],
  }).ending;
}
