import { createEmbeddedFontsElements, createTextElement, documentFontEmbedding } from "../../src";
import { readFontSession } from "../fonts";

import { readDocumentOptions } from "./read-document-options";

import type { DrawnText, LeftOutFace, Text } from "../../src";

/** A text measured and drawn, as the playground's drawing holds it. */
type MeasuredText = {
  /** The text. */
  readonly text: Text;
  /** Its positioned lines, from `textDrawing`. */
  readonly drawn: DrawnText;
};

/** What the drawing writes for its text, and the faces it leaves out. */
type TextDocument = {
  /** The `<defs>` of its embedded fonts; none with the option off or no face embedded. */
  readonly defs: readonly SVGDefsElement[];
  /** The text's group: its `<text>` and its paths. */
  readonly element: SVGGElement;
  /** The faces not embedded, with their reasons. */
  readonly leftOut: readonly LeftOutFace[];
};

/**
 * Creates what the drawing, a document of one text, writes with its options (ADR-0053, ADR-0060):
 * the faces it embeds, each once, from the session's fonts (`documentFontEmbedding`,
 * `createEmbeddedFontsElements`), and the text as its group alone (`createTextElement`), never
 * with an HTML layer; with the option off, no `<defs>`.
 *
 * @kind procedure
 * @param document - page document
 * @param measured - the text and its drawing
 * @returns the `<defs>`, the text's element and the faces left out
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
export function createTextDocument(document: Document, measured: MeasuredText): TextDocument {
  const { drawn, text } = measured;
  const options = readDocumentOptions(document);
  const embedding = documentFontEmbedding([text], readFontSession(document).kept, options);

  return {
    defs: createEmbeddedFontsElements(document, embedding.embedded),
    element: createTextElement(document, text, drawn),
    leftOut: embedding.leftOut,
  };
}
