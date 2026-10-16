import { parseFontFace } from "../../src";

import { addFontFace } from "./add-font-face";

import type { NamedFontFace } from "../../src";

/**
 * Reads a font file's bytes into a face (`parseFontFace`) and keeps it in the session's families
 * (`addFontFace`), the page then able to draw it.
 *
 * @kind procedure
 * @param document - page document
 * @param bytes - the bytes of a font file
 * @param update - redraws the playground once the page can draw the face
 * @returns the face added and its family; undefined for a file that is not a TrueType or OpenType font
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
export function addFontFile(
  document: Document,
  bytes: Readonly<Uint8Array>,
  update: () => void,
): NamedFontFace | undefined {
  const named = parseFontFace(bytes);

  if (named !== undefined) {
    addFontFace(document, named, update);
  }

  return named;
}
