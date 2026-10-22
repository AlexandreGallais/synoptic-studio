import { readShapeKind, selectElement, writeText } from "../page";

/** Title of the panel for each shape kind of the selector. */
const TITLES = new Map([
  ["free", "Free shape"],
  ["none", "No shape selected"],
  ["polygon", "Polygon"],
  ["rectangle", "Rectangle"],
  ["round", "Round shape"],
  ["text", "Text"],
]);

/**
 * Shows the kind of the shape the panel edits: the panel titled with it, the polygon's options
 * shown for a polygon only. The toolbar shows the tool armed instead (`writeArmedTool`, F35).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E02-F35-US-063-a-click-adds-a-shape.md
 */
export function writeShapeKind(document: Document): void {
  const kind = readShapeKind(document);

  selectElement(document, "polygon-options").hidden = kind !== "polygon";
  writeText(document, "shape-title", TITLES.get(kind) ?? kind);
}
