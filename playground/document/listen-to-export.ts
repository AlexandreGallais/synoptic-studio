import { selectElement } from "../page";

import { readExportedSvg } from "./read-exported-svg";

/** The name of the file exported. */
const FILE_NAME = "drawing.svg";

/**
 * Listens to **Export SVG**: the drawing as it is exported (`readExportedSvg`) is saved as a file,
 * which a browser, resvg or Inkscape can open, so that its embedded fonts and its HTML layer can
 * be seen outside the editor.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
export function listenToExport(document: Document): void {
  selectElement(document, "export-svg").addEventListener("click", () => {
    const file = new Blob([readExportedSvg(document)], { type: "image/svg+xml" });
    const link = document.createElement("a");

    link.href = URL.createObjectURL(file);
    link.download = FILE_NAME;
    link.click();
    URL.revokeObjectURL(link.href);
  });
}
