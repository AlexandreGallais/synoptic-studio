import { ascentAtSize, halfLeading } from "../text-block";

import type { FontMetrics } from "../font";
import type { Text } from "../text";

/**
 * How far below the top of its block a browser puts the first baseline of a text's HTML layer,
 * written with the library's line height: half the leading, then the ascent (`REF-CSS-INLINE-3`,
 * §5.3; `DERIV-html-layer`, step 2). It equals the library's own first baseline for a line height
 * chosen; for an automatic one, it lies half the line gap lower (step 4), and a trimmed text's
 * lies lower still, the browser trimming nothing.
 *
 * It holds when the browser uses the ascender and descender of `hhea`, as the library does
 * (`DERIV-html-layer`, steps 3 and 4): on macOS, where Safari and Chrome read `hhea` and Firefox
 * too unless `USE_TYPO_METRICS` is set; elsewhere when `USE_TYPO_METRICS` is unset and the `win`
 * ascent and descent equal `hhea`'s, or when it is set and the typographic ascender and descender
 * equal them. Arial 5.01 (flag unset, `win` 1854 and 434) and Inter 4.1 (flag set, typographic
 * 1984 and −494) meet it; for a font that does not, the browser's first baseline lies
 * `k · ((A_b − D_b) − (A − D)) / 2` lower, a difference accepted.
 *
 * Formula: (λ − (A − D) × s / U) / 2 + A × s / U.
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param text - its size and line height
 * @returns the distance from the top of the layer's block down to its first baseline, in user
 * units
 * @see DERIV-html-layer
 */
export function cssFirstBaselineOffset(metrics: FontMetrics, text: Text): number {
  return halfLeading(metrics, text) + ascentAtSize(metrics, text.size);
}
