import type { FontMetrics } from "../font";
import type { Text } from "../text";

/** A way to wrap a paragraph in its text's box: greedily (`wrappedParagraph`) or balanced. */
export type ParagraphWrap = (
  metrics: FontMetrics,
  paragraph: string,
  text: Text,
) => readonly string[];
