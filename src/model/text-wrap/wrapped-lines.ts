import { textLines } from "../text-block";
import { textInCase } from "../text-case";

import { wrappedParagraph } from "./wrapped-paragraph";

import type { FontMetrics } from "../font";
import type { Text } from "../text";

/**
 * The lines of a text: its content written in its case and cut into paragraphs at each line break
 * typed; a text that fits its content keeps them as its lines, a text in a box wraps each
 * paragraph on its own (`wrappedParagraph`; `DERIV-text-box`, steps 1 and 3).
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param text - its content, case, size, spacing and box
 * @returns its lines, in order; none for an empty content
 * @see DERIV-text-box
 */
export function wrappedLines(metrics: FontMetrics, text: Text): readonly string[] {
  const paragraphs = textLines(textInCase(text.content, text.textCase));

  return text.isInBox
    ? paragraphs.flatMap((paragraph) => wrappedParagraph(metrics, paragraph, text))
    : paragraphs;
}
