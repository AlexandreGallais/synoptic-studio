import { boxCenter } from "../placement";
import { blockOffsetInBox, positionedLines, textFrame } from "../text-layout";

import { lineDecorations } from "./line-decorations";
import { textBackgrounds } from "./text-backgrounds";

import type { FontMetrics } from "../font";
import type { Text } from "../text";
import type { DrawnText } from "./drawn-text";
import type { MeasuredText } from "./measured-text";

/**
 * What the SVG of a text draws (ADR-0052), from what it measures (`textMeasure`): its lines shown
 * placed in what it is laid out in (`textFrame`) — its block, or its box, across the box's width
 * and down its height by its vertical alignment (`blockOffsetInBox`) — the background behind each
 * line when it has one (`textBackgrounds`, `DERIV-text-box`, step 10), the underline and
 * strike-through of each line as rectangles (`DERIV-text-width`, steps 4 to 6;
 * `DERIV-text-box`, step 8), and the middle of its block or its box, which it turns about
 * (`boxCenter`, `DERIV-turned-shape` step 4). An empty text draws nothing; its middle is its
 * position, or its box's.
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param text - its content, font, color, spacing, box and options
 * @param measured - its lines shown and its block, as `textMeasure` gives them for it
 * @returns its block, its middle, its positioned lines, their backgrounds and their decorations
 * @see DERIV-text-box
 */
export function measuredTextDrawing(
  metrics: FontMetrics,
  text: Text,
  measured: MeasuredText,
): DrawnText {
  const { block, lines: shown } = measured;
  const frame = textFrame(text, block);
  const lines = positionedLines(metrics, text, {
    lines: shown,
    top: frame.y + blockOffsetInBox(frame.height, block.height, text.verticalAlignment),
    width: frame.width,
  });

  return {
    backgrounds: textBackgrounds(metrics, text, lines),
    block,
    center: boxCenter(frame),
    decorations: lines.flatMap((line) => lineDecorations(metrics, text, line)),
    lines,
  };
}
