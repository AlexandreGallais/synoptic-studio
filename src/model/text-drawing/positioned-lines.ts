import { lineWidth, textLines } from "../text-block";
import { textInCase } from "../text-case";

import { initialTextPosition } from "./initial-text-position";
import { lineBaseline } from "./line-baseline";
import { lineOffsetInBlock } from "./line-offset-in-block";

import type { FontMetrics } from "../font";
import type { Text } from "../text";
import type { PositionedLine } from "./positioned-line";

/**
 * The lines of a text, placed: its content written in its case and cut into lines, each measured,
 * moved in the block as its alignment says, its `x` its initial text position and its `y` its
 * baseline (`DERIV-text-width`, steps 4 and 5). An empty content has no line. A line tightened
 * past nothing keeps its negative width and may fall outside the block, which is at least 0 wide
 * (`textBlock`).
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param text - its content, case, size, spacing, alignment, direction and position
 * @param blockWidth - the width of its block, from `textBlock`
 * @returns its lines, in order
 * @see DERIV-text-width
 */
export function positionedLines(
  metrics: FontMetrics,
  text: Text,
  blockWidth: number,
): readonly PositionedLine[] {
  const lines = textLines(textInCase(text.content, text.textCase)).map((content) => ({
    content,
    width: lineWidth(metrics, content, text),
  }));

  return lines.map(({ content, width }, index) => {
    const left = text.x + lineOffsetInBlock(blockWidth, width, text.alignment);

    return {
      content,
      left,
      width,
      x: initialTextPosition(left, width, text.direction),
      y: lineBaseline(metrics, text, index),
    };
  });
}
