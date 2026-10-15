import { maximum } from "../../math";

import type { LinesEnding } from "./lines-ending";

/**
 * The most tokens a line that fits holds, at least one: how far back from an end the dynamic
 * programming looks, so that it keeps only that many earlier results (`DERIV-text-box`, step 9).
 *
 * Formula: max(1, max{ j − i : the tokens i to j − 1 fit on a line }).
 *
 * @kind domain
 * @param ending - for each index from 0 to the number of tokens, the lines that fit and end there
 * @returns the most tokens a line holds, 1 when none fits
 * @see DERIV-text-box
 */
export function longestLineSpan(ending: readonly LinesEnding[]): number {
  return maximum(
    ending.flatMap(({ lines }, end) => lines.map(({ start }) => end - start)),
    1,
  );
}
