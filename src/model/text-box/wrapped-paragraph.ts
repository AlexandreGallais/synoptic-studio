import { isLineFitting } from "./is-line-fitting";
import { paragraphTokens } from "./paragraph-tokens";
import { withoutTrailingSpaces } from "./without-trailing-spaces";
import { wordBrokenAtCharacters } from "./word-broken-at-characters";

import type { LineWrap } from "./line-wrap";
import type { FontMetrics } from "../font";
import type { Text } from "../text";

/** How a token is placed on a line: whether a line fits the box, and whether a word is broken. */
type TokenPlacement = {
  /** Whether a line fits the box (`isLineFitting`). */
  readonly isFitting: (line: string) => boolean;
  /** Whether a token is a word too long for a line of its own, to break at a character. */
  readonly isBroken: (token: string) => boolean;
};

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
  return wrap.line === "" ? wrap.lines : [...wrap.lines, withoutTrailingSpaces(wrap.line)];
}

/**
 * A paragraph wrapped with one more token: the token joins the line being filled when the line
 * with it fits; otherwise it starts the next line, and a word too long for a line of its own is
 * broken at a character when the text says so, or stays whole (`DERIV-text-box`, steps 3, 4).
 *
 * @kind domain
 * @param wrap - the paragraph wrapped so far
 * @param token - the next token
 * @param placement - the fit of a line and the long-word option
 * @returns the paragraph wrapped with the token
 * @see DERIV-text-box
 */
function wrapWithToken(wrap: LineWrap, token: string, placement: TokenPlacement): LineWrap {
  if (wrap.line !== "" && placement.isFitting(wrap.line + token)) {
    return { line: wrap.line + token, lines: wrap.lines };
  }

  const lines = linesBeforeWrap(wrap);

  return placement.isBroken(token)
    ? wordBrokenAtCharacters({ line: "", lines }, token, placement.isFitting)
    : { line: token, lines };
}

/**
 * The lines of a paragraph wrapped greedily in its text's box: each token, in order, is placed on
 * an empty line, joins the line when the line with it fits (`isLineFitting`), and otherwise ends
 * the line and starts the next; a word too long for a line of its own is broken at a character or
 * moved whole, as the text says; a line ended by a wrap loses its trailing spaces, the last keeps
 * those typed (`DERIV-text-box`, steps 1 to 4).
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
  const placement: TokenPlacement = {
    isBroken: (token) => text.longWord === "break" && !isLineFitting(metrics, token, text),
    isFitting: (line) => isLineFitting(metrics, line, text),
  };
  const wrap = paragraphTokens(paragraph).reduce<LineWrap>(
    (state, token) => wrapWithToken(state, token, placement),
    { line: "", lines: [] },
  );

  return [...wrap.lines, wrap.line];
}
