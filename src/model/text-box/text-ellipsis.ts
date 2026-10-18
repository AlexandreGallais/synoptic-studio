import type { FontMetrics } from "../font";

/** The ellipsis U+2026. */
const ELLIPSIS = "\u{2026}";

/** Its code point. */
const ELLIPSIS_CODE_POINT = 0x20_26;

/** What a face without the ellipsis writes instead: three full stops. */
const THREE_DOTS = "...";

/**
 * What ends a line cut: the ellipsis character U+2026, "or three dots \"...\" if the ellipsis
 * character is unavailable", that is when the face maps no glyph to it (`DERIV-text-box`, step 6).
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @returns "…", or "..." for a face without it
 * @see REF-CSS-OVERFLOW-3
 */
export function textEllipsis(metrics: FontMetrics): string {
  return metrics.advances[ELLIPSIS_CODE_POINT] === undefined ? THREE_DOTS : ELLIPSIS;
}
