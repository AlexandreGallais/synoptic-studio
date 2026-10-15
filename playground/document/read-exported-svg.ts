/**
 * Reads a copy of the drawing as a file's text, the dashed frames of the selection removed.
 *
 * @kind procedure
 * @param copy - a copy of the canvas's `<svg>`, changed here
 * @returns its text
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
function readDrawingText(copy: Element): string {
  const serializer = new XMLSerializer();

  for (const frame of copy.querySelectorAll(".text-frame")) {
    frame.remove();
  }

  return serializer.serializeToString(copy);
}

/**
 * Reads the drawing as it is exported: the canvas's `<svg>`, its embedded fonts and its text,
 * without the dashed frame of the selection, which is the editor's and not the drawing's.
 *
 * @kind procedure
 * @param document - page document
 * @returns the SVG file's text; empty when nothing is drawn
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
export function readExportedSvg(document: Document): string {
  const copy = document.querySelector("#canvas svg")?.cloneNode(true);

  return copy instanceof Element ? readDrawingText(copy) : "";
}
