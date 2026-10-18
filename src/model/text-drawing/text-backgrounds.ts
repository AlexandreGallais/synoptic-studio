import { contentAreaHeight } from "../text-block";

import { contentAreaTop } from "./content-area-top";
import { isLineWithLength } from "./is-line-with-length";

import type { LineDecoration } from "./line-decoration";
import type { FontMetrics } from "../font";
import type { Text } from "../text";
import type { PositionedLine } from "../text-layout";

/**
 * The background of one line: its content area, from the font's ascender to its descender
 * (`contentAreaHeight`, `REF-CSS-INLINE-3`, §6.4), as long as the line drawn, its top an ascent
 * above the baseline (`contentAreaTop`), whatever the line height or the trim (`DERIV-text-box`,
 * step 10).
 *
 * Formula: x = left, y = baseline − A × s / U, width = w, height = (A − D) × s / U.
 *
 * @kind domain
 * @param metrics - the font face's metrics
 * @param size - the text's size
 * @param line - the line, placed
 * @returns its background's rectangle
 * @see DERIV-text-box
 */
function lineBackground(metrics: FontMetrics, size: number, line: PositionedLine): LineDecoration {
  return {
    height: contentAreaHeight(metrics, size),
    width: line.width,
    x: line.left,
    y: contentAreaTop(metrics, size, line.y),
  };
}

/**
 * The backgrounds of a text's lines, as a highlighter, when it has one: one rectangle per line
 * with a length (`isLineWithLength`), its ellipsis included, none for an empty line or one tightened
 * past nothing (`DERIV-text-box`, step 10).
 *
 * @kind domain
 * @param metrics - the font face's metrics
 * @param text - its size and whether it has a background
 * @param lines - its lines, placed
 * @returns the rectangles, line by line
 * @see DERIV-text-box
 */
export function textBackgrounds(
  metrics: FontMetrics,
  text: Text,
  lines: readonly PositionedLine[],
): readonly LineDecoration[] {
  return text.hasBackground
    ? lines
        .filter((line) => isLineWithLength(line))
        .map((line) => lineBackground(metrics, text.size, line))
    : [];
}
