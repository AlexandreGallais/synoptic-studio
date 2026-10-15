import { selectElement } from "../page";

import { readExportedSvg } from "./read-exported-svg";

import type { Text } from "../../src";

/** The name of the file exported. */
const FILE_NAME = "drawing.svg";

/**
 * Listens to **Export SVG**: the drawing as it is exported (`readExportedSvg`) is saved as a file,
 * which a browser, resvg or Inkscape can open, so that its embedded fonts, written only in that
 * file (ADR-0060), can be seen outside the editor.
 *
 * @kind procedure
 * @param document - page document
 * @param readTexts - reads the texts of the drawing, as the page holds them
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
export function listenToExport(document: Document, readTexts: () => readonly Text[]): void {
  selectElement(document, "export-svg").addEventListener("click", () => {
    const file = new Blob([readExportedSvg(document, readTexts())], { type: "image/svg+xml" });
    const link = document.createElement("a");

    link.href = URL.createObjectURL(file);
    link.download = FILE_NAME;
    link.click();
    URL.revokeObjectURL(link.href);
  });
}
