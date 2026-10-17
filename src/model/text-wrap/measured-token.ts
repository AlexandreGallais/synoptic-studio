import { measuredLine } from "../text-block";

import { withoutTrailingSpaces } from "./without-trailing-spaces";

import type { TokenMeasure } from "./token-measure";
import type { FontMetrics } from "../font";

/**
 * What a token's width is made of, measured once: the token with the spaces after it, which join
 * a line that goes on, and without them, as it ends a line and is measured for fit
 * (`DERIV-text-box`, step 2).
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param token - a token of a paragraph, in the text's case
 * @returns its measures with and without its trailing spaces
 * @see DERIV-text-box
 */
export function measuredToken(metrics: FontMetrics, token: string): TokenMeasure {
  return {
    visible: measuredLine(metrics, withoutTrailingSpaces(token)),
    whole: measuredLine(metrics, token),
  };
}
