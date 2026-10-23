import type { FontMetrics, Size, Text } from "../../src";

/**
 * Reads the block a text is shown and placed with in the playground (F40): its measured block; for an
 * empty text fitting its content, whose block has no area and draws nothing, a block 1 wide and
 * one line tall (`textLineHeight`), where its first character will go, so that the designer sees
 * where the text placed stands before typing it (agent's choice, US-075). A text in a box keeps
 * its block, its frame being the box.
 *
 * Formula: 1 × λ for an empty content fitting it, λ the text's line height; the block otherwise.
 *
 * @kind procedure
 * @param metrics - the metrics of the face it is drawn with
 * @param text - its content, its size, its line height and whether it sits in a box
 * @param block - what it measures, `textMeasure`'s block
 * @returns 1 × λ, or the block given
 * @see docs/domain/interaction.md#_1-tools
 */
export function readShownTextBlock(metrics: FontMetrics, text: Text, block: Size): Size {
  return block;
}
