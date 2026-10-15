import { formatCssPixels } from "./format-css-pixels";

import type { CssDeclaration } from "./css-declaration";
import type { HtmlLayer, Text, TextVerticalAlignment } from "../../model";

/** Where a flex column packs its item for each vertical alignment of a text in its box. */
const PACKING: Readonly<Record<TextVerticalAlignment, string>> = {
  bottom: "flex-end",
  middle: "center",
  top: "flex-start",
};

/**
 * The style of the outer `div` of a text's HTML layer, as wide and tall as its `foreignObject`:
 * in a box, a flex column that places the block of lines down the box by its vertical alignment,
 * a block taller than the box overflowing both edges equally in the middle, as the library's does
 * (`REF-MDN-JUSTIFY-CONTENT`: the "desired alignment will be implemented" even when it
 * overflows; `blockOffsetInBox`); a text that fits its content, only its size.
 *
 * @kind format
 * @param text - whether it sits in a box, and its vertical alignment
 * @param layer - its frame
 * @returns the declarations, in order
 * @see REF-MDN-JUSTIFY-CONTENT
 */
export function htmlLayerToFrameStyle(text: Text, layer: HtmlLayer): readonly CssDeclaration[] {
  const size: readonly CssDeclaration[] = [
    ["width", formatCssPixels(layer.frame.width)],
    ["height", formatCssPixels(layer.frame.height)],
  ];

  return text.isInBox
    ? [
        ...size,
        ["display", "flex"],
        ["flex-direction", "column"],
        ["justify-content", PACKING[text.verticalAlignment]],
      ]
    : size;
}
