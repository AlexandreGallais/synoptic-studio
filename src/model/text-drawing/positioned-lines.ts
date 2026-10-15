import { lineWidth } from "../text-block";

import { initialTextPosition } from "./initial-text-position";
import { lineBaseline } from "./line-baseline";
import { lineOffsetInBlock } from "./line-offset-in-block";

import type { FontMetrics } from "../font";
import type { Text } from "../text";
import type { PositionedLine } from "./positioned-line";
import type { ShownBlock } from "./shown-block";

/**
 * The lines of a text, placed: its lines shown (`shownLines`), each measured,
 * moved in the block as its alignment says, its `x` its initial text position and its `y` its
 * baseline (`DERIV-text-width`, steps 4 and 5). An empty content has no line. A line tightened
 * past nothing keeps its negative width and may fall outside the block, which is at least 0 wide
 * (`textBlock`).
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param text - its content, case, size, spacing, alignment, direction and position
 * @param block - its lines shown (`shownLines`) and the width of their block (`textBlock`)
 * @returns its lines, in order
 * @see DERIV-text-width
 */
export function positionedLines(
  metrics: FontMetrics,
  text: Text,
  block: ShownBlock,
): readonly PositionedLine[] {
  const lines = block.lines.map((content) => ({
    content,
    width: lineWidth(metrics, content, text),
  }));

  return lines.map(({ content, width }, index) => {
    const left = text.x + lineOffsetInBlock(block.width, width, text.alignment);

    return {
      content,
      left,
      width,
      x: initialTextPosition(left, width, text.direction),
      y: lineBaseline(metrics, text, index),
    };
  });
}
