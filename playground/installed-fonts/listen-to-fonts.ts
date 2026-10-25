import { writeDefaultFontFaces } from "../default-font";
import { listenToFontImport } from "../fonts";

import { listenToInstalledFonts } from "./listen-to-installed-fonts";

/**
 * Listens to the font controls of the Text tool, once Inter's faces the browser draws are kept
 * (`writeDefaultFontFaces`, US-053): **Import a font** (`listenToFontImport`), and
 * **Installed fonts** with the font list (`listenToInstalledFonts`).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see US-036
 */
export function listenToFonts(document: Document, update: () => void): void {
  writeDefaultFontFaces(document);
  listenToFontImport(document, update);
  listenToInstalledFonts(document, update);
}
