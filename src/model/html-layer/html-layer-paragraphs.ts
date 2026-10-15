import { textLines } from "../text-block";
import { textInCase } from "../text-case";

import type { Text } from "../text";

/** U+200B ZERO WIDTH SPACE: text, so that its line is laid out, and a format character the library measures as absent. */
const ZERO_WIDTH_SPACE = "\u{200B}";

/**
 * The paragraphs of a text's HTML layer: its content written in its case, cut at each line break
 * typed (`textInCase`, `textLines`), the browser wrapping them itself. A last paragraph left
 * empty, a content ending with a line break, holds a zero-width space: a line box holding no text
 * and not ending with a forced line break is a phantom line box, "treated as not existing"
 * (`REF-CSS-INLINE-3`, §2.3), so that the browser would lay out one line fewer than the library,
 * which draws that last line empty; a paragraph empty before another ends with the line feed
 * between them and is laid out.
 *
 * @kind domain
 * @param text - its content and case
 * @returns its paragraphs, in order; none for an empty content
 * @see REF-CSS-INLINE-3
 */
export function htmlLayerParagraphs(text: Text): readonly string[] {
  const paragraphs = textLines(textInCase(text.content, text.textCase));
  const last = paragraphs.length - 1;

  return paragraphs.map((paragraph, index) =>
    index === last && paragraph === "" ? ZERO_WIDTH_SPACE : paragraph,
  );
}
