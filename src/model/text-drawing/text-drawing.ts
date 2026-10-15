import { textBlock } from "../text-block";

import { lineDecorations } from "./line-decorations";
import { positionedLines } from "./positioned-lines";

import type { FontMetrics } from "../font";
import type { Text } from "../text";
import type { DrawnText } from "./drawn-text";

/**
 * What the SVG of a text draws (ADR-0052): its lines placed in its block, and the underline and
 * strike-through of each line as rectangles (`DERIV-text-width`, steps 4 to 6). An empty text
 * draws nothing.
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param text - its content, font, color, spacing and options
 * @returns its block, its positioned lines and their decorations
 * @see DERIV-text-width
 */
export function textDrawing(metrics: FontMetrics, text: Text): DrawnText {
  const block = textBlock(metrics, text);
  const lines = positionedLines(metrics, text, block.width);

  return {
    block,
    decorations: lines.flatMap((line) => lineDecorations(metrics, text, line)),
    lines,
  };
}
