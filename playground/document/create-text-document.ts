import { createTextElement, documentFontEmbedding } from "../../src";
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

/** What the drawing writes for its text, and the faces a file written from it would leave out. */
type TextDocument = {
  /** The text's group: its `<text>` and its paths. */
  readonly element: SVGGElement;
  /** The faces a file exported would not embed, with their reasons. */
  readonly leftOut: readonly LeftOutFace[];
};

/**
 * Creates what the drawing, a document of one text, writes on screen (ADR-0060): the text as its
 * group alone (`createTextElement`), never its embedded fonts, which only a file exported holds
 * (`readExportedSvg`); with Embed fonts on, the faces that file would leave out, from the session's
 * fonts (`documentFontEmbedding`), so that the editor says so while the text is edited.
 *
 * @kind procedure
 * @param document - page document
 * @param measured - the text and its drawing
 * @returns the text's element and the faces left out
 * @see docs/backlog/stories/E15-F17-US-045-fonts-embedded-at-export.md
 */
export function createTextDocument(document: Document, measured: MeasuredText): TextDocument {
  const { drawn, text } = measured;
  const options = readDocumentOptions(document);
  const { leftOut } = documentFontEmbedding([text], readFontSession(document).kept, options);

  return { element: createTextElement(document, text, drawn), leftOut };
}
