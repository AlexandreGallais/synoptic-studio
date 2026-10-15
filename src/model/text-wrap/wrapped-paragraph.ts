import { joinedLineMeasure, measuredLine } from "../text-block";

import { isMeasureFitting } from "./is-measure-fitting";
import { measuredToken } from "./measured-token";
import { paragraphTokens } from "./paragraph-tokens";
import { withoutTrailingSpaces } from "./without-trailing-spaces";
import { wordBrokenAtCharacters } from "./word-broken-at-characters";

import type { LineWrap } from "./line-wrap";
import type { TextInBox } from "./text-in-box";
import type { FontMetrics } from "../font";
import type { Text } from "../text";

/**
 * The lines of a paragraph once its line being filled is ended by a wrap: that line joins them
 * without its trailing spaces, unless it is still empty.
 *
 * @kind domain
 * @param wrap - the paragraph wrapped so far
 * @returns the lines before the next line
 * @see DERIV-text-box
 */
function linesBeforeWrap(wrap: LineWrap): readonly string[] {
  const { text } = wrap.line;

  return text === "" ? wrap.lines : [...wrap.lines, withoutTrailingSpaces(text)];
}

/**
 * A paragraph wrapped with one more token: the token joins the line being filled when the line
 * with it, its trailing spaces left out, fits; otherwise it starts the next line, and a word too
 * long for a line of its own is broken at a character when the text says so, or stays whole
 * (`DERIV-text-box`, steps 2 to 4). The line's measure grows by the token's, measured once
 * (`joinedLineMeasure`).
 *
 * @kind domain
 * @param wrap - the paragraph wrapped so far
 * @param token - the next token
 * @param context - the text and its face's metrics
 * @returns the paragraph wrapped with the token
 * @see DERIV-text-box
 */
function wrapWithToken(wrap: LineWrap, token: string, context: TextInBox): LineWrap {
  const { metrics, text } = context;
  const { visible, whole } = measuredToken(metrics, token);

  if (
    wrap.line.text !== "" &&
    isMeasureFitting(metrics, joinedLineMeasure(metrics, wrap.line, visible), text)
  ) {
    return { line: joinedLineMeasure(metrics, wrap.line, whole), lines: wrap.lines };
  }

  const next = { line: measuredLine(metrics, ""), lines: linesBeforeWrap(wrap) };

  return text.longWord === "break" && !isMeasureFitting(metrics, visible, text)
    ? wordBrokenAtCharacters(next, token, context)
    : { line: whole, lines: next.lines };
}

/**
 * The lines of a paragraph wrapped greedily in its text's box: each token, in order, is placed on
 * an empty line, joins the line when the line with it fits, and otherwise ends the line and starts
 * the next; a word too long for a line of its own is broken at a character or moved whole, as the
 * text says; a line ended by a wrap loses its trailing spaces, the last keeps those typed
 * (`DERIV-text-box`, steps 1 to 4). Each token is measured once, so that wrapping costs as much as
 * measuring the paragraph.
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param paragraph - a paragraph of the text, in its case
 * @param text - its size, spacing, box and long-word option
 * @returns its lines, in order; one empty line for an empty paragraph
 * @see DERIV-text-box
 */
export function wrappedParagraph(
  metrics: FontMetrics,
  paragraph: string,
  text: Text,
): readonly string[] {
  const context = { metrics, text };
  const wrap = paragraphTokens(paragraph).reduce<LineWrap>(
    (state, token) => wrapWithToken(state, token, context),
    { line: measuredLine(metrics, ""), lines: [] },
  );

  return [...wrap.lines, wrap.line.text];
}
