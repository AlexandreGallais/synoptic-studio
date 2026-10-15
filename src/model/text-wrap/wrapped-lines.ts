import { textLines } from "../text-block";
import { textInCase } from "../text-case";

import type { ParagraphWrap } from "./paragraph-wrap";
import type { FontMetrics } from "../font";
import type { Text } from "../text";

/**
 * The lines of a text: its content written in its case and cut into paragraphs at each line break
 * typed; a text that fits its content keeps them as its lines (F05), a text in a box wraps each
 * paragraph on its own, greedily or balanced (`DERIV-text-box`, steps 1, 3 and 9).
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param text - its content, case, size, spacing and box
 * @param wrapParagraph - how each paragraph is wrapped in a box: `wrappedParagraph` or
 * `balancedParagraph`
 * @returns its lines, in order; none for an empty content
 * @see DERIV-text-box
 */
export function wrappedLines(
  metrics: FontMetrics,
  text: Text,
  wrapParagraph: ParagraphWrap,
): readonly string[] {
  const paragraphs = textLines(textInCase(text.content, text.textCase));

  return text.isInBox
    ? paragraphs.flatMap((paragraph) => wrapParagraph(metrics, paragraph, text))
    : paragraphs;
}
