import { createEmbeddedFontsElements, documentFontEmbedding } from "../../src";
import { readFontSession } from "../fonts";

import { readDocumentOptions } from "./read-document-options";

import type { Text } from "../../src";

/** What the editor draws over the shapes, never exported: frames, boxes and handles. */
const EDITOR_MARKS = ".text-frame, .box, .turned-box, .straight-box, .handle, .halo, .curve-handle";

/**
 * Reads a copy of the drawing as a file's text, without what the editor draws over its shapes:
 * the dashed frames and boxes of the selection and the handles of its points (F35).
 *
 * @kind procedure
 * @param copy - a copy of the canvas's `<svg>`, changed here
 * @returns its text
 * @see docs/backlog/stories/E15-F15-US-036-import-choose-and-embed-a-font.md
 */
function readDrawingText(copy: Element): string {
  const serializer = new XMLSerializer();

  for (const mark of copy.querySelectorAll(EDITOR_MARKS)) {
    mark.remove();
  }

  return serializer.serializeToString(copy);
}

/**
 * Reads the drawing as it is exported: the canvas's `<svg>` and its text, every shape in the order
 * drawn (F35), without the frames, boxes and handles of the selection, which are the editor's and
 * not the drawing's; with Embed fonts on, the faces
 * its texts are measured with embedded first, in a `<defs>` (`documentFontEmbedding`,
 * `createEmbeddedFontsElements`), which only a file written holds, never the drawing on screen
 * (ADR-0060, US-045).
 *
 * @kind procedure
 * @param document - page document
 * @param texts - the texts of the drawing
 * @returns the SVG file's text; empty when nothing is drawn
 * @see docs/backlog/stories/E15-F17-US-045-fonts-embedded-at-export.md
 */
export function readExportedSvg(document: Document, texts: readonly Text[]): string {
  const copy = document.querySelector("#canvas svg")?.cloneNode(true);

  if (!(copy instanceof Element)) {
    return "";
  }

  const options = readDocumentOptions(document);
  const { embedded } = documentFontEmbedding(texts, readFontSession(document).kept, options);

  copy.prepend(...createEmbeddedFontsElements(document, embedded));

  return readDrawingText(copy);
}
