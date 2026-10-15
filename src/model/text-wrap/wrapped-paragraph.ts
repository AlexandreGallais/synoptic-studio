import { scan } from "../../math";
import { measuredLine } from "../text-block";

import { lineWrapWithToken } from "./line-wrap-with-token";
import { paragraphTokens } from "./paragraph-tokens";

import type { LineWrap } from "./line-wrap";
import type { FontMetrics } from "../font";
import type { Text } from "../text";

/**
 * The lines of a paragraph wrapped greedily in its text's box: each token, in order, is placed on
 * an empty line, joins the line when the line with it fits, and otherwise ends the line and starts
 * the next; a word too long for a line of its own is broken at a character or moved whole, as the
 * text says; a line ended by a wrap loses its trailing spaces, the last keeps those typed
 * (`DERIV-text-box`, steps 1 to 4; `lineWrapWithToken`). Each token is measured once, and the steps are a scan
 * (`scan`), so that wrapping costs about as much as measuring the paragraph.
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
  const steps = scan<LineWrap, string>(
    paragraphTokens(paragraph),
    (wrap, token) => lineWrapWithToken(wrap, token, context),
    { ended: [], line: measuredLine(metrics, "") },
  );

  return [...steps.flatMap(({ ended }) => ended), steps.at(-1)?.line.text ?? ""];
}
