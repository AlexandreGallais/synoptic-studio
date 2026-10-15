import { isLineFitting } from "./is-line-fitting";
import { paragraphTokens } from "./paragraph-tokens";
import { withoutTrailingSpaces } from "./without-trailing-spaces";

import type { LineWrap } from "./line-wrap";
import type { FontMetrics } from "../font";
import type { Text } from "../text";

/**
 * The lines of a paragraph wrapped greedily in its text's box: each token, in order, is placed on
 * an empty line, joins the line when the line with it fits (`isLineFitting`), and otherwise ends
 * the line and starts the next; a line ended by a wrap loses its trailing spaces, the last keeps
 * those typed (`DERIV-text-box`, steps 1 to 3).
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param paragraph - a paragraph of the text, in its case
 * @param text - its size, spacing and box
 * @returns its lines, in order; one empty line for an empty paragraph
 * @see DERIV-text-box
 */
export function wrappedParagraph(
  metrics: FontMetrics,
  paragraph: string,
  text: Text,
): readonly string[] {
  const wrap = paragraphTokens(paragraph).reduce<LineWrap>(
    (state, token) =>
      state.line === "" || isLineFitting(metrics, state.line + token, text)
        ? { line: state.line + token, lines: state.lines }
        : { line: token, lines: [...state.lines, withoutTrailingSpaces(state.line)] },
    { line: "", lines: [] },
  );

  return [...wrap.lines, wrap.line];
}
