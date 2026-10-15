import { textLineHeight } from "../text-block";
import { blockHeight } from "../text-box";
import { blockOffsetInBox, firstBaselineOffset, textFrame } from "../text-layout";

import { cssFirstBaselineOffset } from "./css-first-baseline-offset";

import type { FontMetrics } from "../font";
import type { TurnedBox } from "../placement";
import type { Text } from "../text";
import type { DrawnText } from "../text-drawing";

/**
 * The rectangle of a text's HTML layer (`DERIV-html-layer`, step 4): as wide as its block and as
 * tall as its lines untrimmed (`n` line heights) for a text that fits its content, its box's width
 * and height in a box; moved down by the library's first baseline less the browser's, so that
 * they meet. The library's first baseline lies its block's offset down its frame
 * (`blockOffsetInBox`) and its first baseline offset (`firstBaselineOffset`) below the frame's
 * top; the browser's, its untrimmed block's offset down the layer, placed by the same vertical
 * alignment, and the browser's first baseline offset (`cssFirstBaselineOffset`). Automatic and
 * untrimmed, the layer is raised by half the line gap: Arial 16, 0.26171875; trimmed, 3.28515625;
 * a line height chosen, untrimmed, not moved. The browser's own wrapping is not known: the lines
 * are assumed the library's.
 *
 * Formula: y = y_t + (H − h(n)) · v + b − (H_l − n · λ) · v − b_css, v = 0, ½ or 1;
 * H_l = n · λ fitting its content (no offset), the box's height in a box.
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param text - its position, box, line height, trim and vertical alignment
 * @param drawn - its block and its lines, from `textDrawing`
 * @returns the layer's position, width and height, in user units
 * @see DERIV-html-layer
 */
export function htmlLayerFrame(
  metrics: FontMetrics,
  text: Text,
  drawn: DrawnText,
): Omit<TurnedBox, "angle"> {
  const frame = textFrame(text, drawn.block);
  const line = textLineHeight(metrics, text);
  const untrimmed = blockHeight({ firstLine: line, line }, drawn.lines.length);
  const height = text.isInBox ? text.boxHeight : untrimmed;
  const libraryBaseline =
    blockOffsetInBox(frame.height, drawn.block.height, text.verticalAlignment) +
    firstBaselineOffset(metrics, text);
  const cssBaseline =
    blockOffsetInBox(height, untrimmed, text.verticalAlignment) +
    cssFirstBaselineOffset(metrics, text);

  return { height, width: frame.width, x: frame.x, y: frame.y + libraryBaseline - cssBaseline };
}
