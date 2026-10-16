import { DEFAULT_FONT } from "../../src";
import { readFontSession, writeFontSession } from "../fonts";
import { writeText } from "../page";

import { DEFAULT_FONT_MISSING } from "./default-font-missing";
import { readDrawnDefaultFaces } from "./read-drawn-default-faces";

/**
 * Keeps of Inter's shipped faces those the browser draws as their metrics measure them
 * (`readDrawnDefaultFaces`, US-053), as the page opens: the weight list then offers those alone,
 * and a face read from a file, imported or installed, is added to them. When none is drawn, Inter
 * stays in the font list without faces, a text in it is not measured, and the page says Inter is
 * missing and how to get it. A browser that cannot measure on a canvas keeps the 18 faces.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/shapes.md#text
 */
export function writeDefaultFontFaces(document: Document): void {
  const faces = readDrawnDefaultFaces(document);

  if (faces === undefined) {
    return;
  }

  const session = readFontSession(document);
  const [inter = DEFAULT_FONT, ...others] = session.kept;

  writeFontSession(document, { ...session, kept: [{ ...inter, faces }, ...others] });

  if (faces.length === 0) {
    writeText(document, "font-message", DEFAULT_FONT_MISSING);
  }
}
