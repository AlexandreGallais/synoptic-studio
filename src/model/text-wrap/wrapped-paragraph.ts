import { scan } from "../../math";
import { measuredLine } from "../text-block";

import { lineWrapWithToken } from "./line-wrap-with-token";
import { paragraphTokens } from "./paragraph-tokens";
import { withoutTrailingSpaces } from "./without-trailing-spaces";

import type { LineWrap } from "./line-wrap";
import type { FontMetrics } from "../font";
import type { Text } from "../text";

/**
 * The lines of a paragraph wrapped greedily in its text's box: each token, in order, is placed on
 * an empty line, joins the line when the line with it fits, and otherwise ends the line and starts
 * the next; a word too long for a line of its own stays whole on it, overflowing; a line ended
 * by a wrap loses its trailing spaces, and so does the last, so that a space typed at the end
 * neither places the line nor lengthens its background (`DERIV-text-box`, steps 1 to 4, step 2 as
 * decided for US-049; `lineWrapWithToken`). Each token is measured once, and the steps are a scan (`scan`), so
 * that wrapping costs about as much as measuring the paragraph.
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
  const context = { metrics, text };
  const steps = scan<LineWrap, string>(
    paragraphTokens(paragraph),
    (wrap, token) => lineWrapWithToken(wrap, token, context),
    { ended: [], line: measuredLine(metrics, "") },
  );

  return [
    ...steps.flatMap(({ ended }) => ended),
    withoutTrailingSpaces(steps.at(-1)?.line.text ?? ""),
  ];
}
