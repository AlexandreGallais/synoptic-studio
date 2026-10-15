import { listenToFontImport } from "./listen-to-font-import";

/**
 * Listens to the font controls of the Text tool: **Import a font** (`listenToFontImport`).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
export function listenToFonts(document: Document, update: () => void): void {
  listenToFontImport(document, update);
}
