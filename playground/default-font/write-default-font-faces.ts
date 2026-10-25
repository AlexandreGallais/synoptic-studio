import { DEFAULT_FONT } from "../../src";
import { readFontSession, writeFontSession } from "../fonts";
import { writeText } from "../page";

import { DEFAULT_FONT_MISSING } from "./default-font-missing";
import { readDrawnDefaultFaces } from "./read-drawn-default-faces";

/**
 * Keeps of Inter's shipped faces those the browser draws as their metrics measure them
 * (`readDrawnDefaultFaces`, US-053), as the page opens: the weight list then offers those alone,
 * and a face read from a file, imported or installed, is added to them. When none is drawn, Inter
 * leaves the font list (US-056): no font is available, a text is not drawn, and the page says to
 * click Installed fonts first, a font being chosen as soon as one is read, or to import a font.
 * A browser that cannot measure on a canvas keeps the 18 faces.
 *
 * @kind procedure
 * @param document - page document
 * @see RULE-shapes-text
 */
export function writeDefaultFontFaces(document: Document): void {
  const faces = readDrawnDefaultFaces(document);

  if (faces === undefined) {
    return;
  }

  const session = readFontSession(document);
  const [inter = DEFAULT_FONT, ...others] = session.kept;

  if (faces.length === 0) {
    writeFontSession(document, { ...session, kept: others });
    writeText(document, "font-message", DEFAULT_FONT_MISSING);

    return;
  }

  writeFontSession(document, { ...session, kept: [{ ...inter, faces }, ...others] });
}
