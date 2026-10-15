import { maximum } from "../../math";
import { lineHeight, lineWidth } from "../text-block";
import { shownLines } from "../text-box";

import type { Size } from "../../geometry";
import type { FontMetrics } from "../font";
import type { Text } from "../text";

/**
 * The block of a text, measured from its font face's metrics: its lines shown (`shownLines`),
 * those of its content written in its case or, in a box, those its box shows with their
 * ellipses, as wide as its widest line and as tall as its lines' heights together
 * (`DERIV-text-width`, step 4). An empty content has no line: a block 0 wide and 0 tall; lines
 * narrower than nothing, tightened by a negative spacing, give a block 0 wide.
 *
 * Formula: width = max(0, widths of the lines); height = k × lineHeight for k lines.
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param text - its content, case, size and spacing
 * @returns the block's width and height, in user units
 * @see DERIV-text-width
 */
export function textBlock(metrics: FontMetrics, text: Text): Size {
  const lines = shownLines(metrics, text);

  return {
    height: lines.length * lineHeight(metrics, text.size),
    width: maximum(
      lines.map((line) => lineWidth(metrics, line, text)),
      0,
    ),
  };
}
