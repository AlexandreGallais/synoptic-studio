import { listenToFontImport } from "../fonts";

import { listenToInstalledFonts } from "./listen-to-installed-fonts";

/**
 * Listens to the font controls of the Text tool: **Import a font** (`listenToFontImport`), and
 * **Installed fonts** with the font list (`listenToInstalledFonts`).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
export function listenToFonts(document: Document, update: () => void): void {
  listenToFontImport(document, update);
  listenToInstalledFonts(document, update);
}
