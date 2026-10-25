import { textLineHeight } from "editor";

import type { FontMetrics, Size, Text } from "editor";

/** The width an empty text is shown with, a caret's, in user units. */
const EMPTY_TEXT_WIDTH = 1;

/**
 * Reads the block a text is shown and placed with in the playground: its measured block; for an
 * empty text fitting its content, whose block has no area and draws nothing, a block 1 wide and
 * one line tall (`textLineHeight`), where its first character will go, so that the designer sees
 * where the text placed stands before typing it (agent's choice). A text in a box keeps
 * its block, its frame being the box.
 *
 * Formula: 1 × λ for an empty content fitting it, λ the text's line height; the block otherwise.
 *
 * @kind procedure
 * @param metrics - the metrics of the face it is drawn with
 * @param text - its content, its size, its line height and whether it sits in a box
 * @param block - what it measures, `textMeasure`'s block
 * @returns 1 × λ, or the block given
 */
export function readShownTextBlock(metrics: FontMetrics, text: Text, block: Size): Size {
  const isEmpty = text.content === "" && !text.isInBox;

  return isEmpty ? { height: textLineHeight(metrics, text), width: EMPTY_TEXT_WIDTH } : block;
}
