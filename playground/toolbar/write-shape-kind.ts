import { readShapeKind, selectElement, writeText } from "../page";

/** Title of the panel for each shape kind of the selector. */
const TITLES = new Map([
  ["free", "Free shape"],
  ["polygon", "Polygon"],
  ["rectangle", "Rectangle"],
  ["round", "Round shape"],
  ["text", "Text"],
]);

/**
 * Shows the shape kind chosen: the toolbar button of that kind pressed, the panel titled with it,
 * the polygon's options shown for a polygon only. A free shape presses no button.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E01-RET-001-retrospective-e01.md
 */
export function writeShapeKind(document: Document): void {
  const kind = readShapeKind(document);

  selectElement(document, "tool-rectangle").setAttribute(
    "aria-pressed",
    String(kind === "rectangle"),
  );

  selectElement(document, "tool-polygon").setAttribute("aria-pressed", String(kind === "polygon"));
  selectElement(document, "tool-round").setAttribute("aria-pressed", String(kind === "round"));
  selectElement(document, "tool-text").setAttribute("aria-pressed", String(kind === "text"));
  selectElement(document, "polygon-options").hidden = kind !== "polygon";
  writeText(document, "shape-title", TITLES.get(kind) ?? kind);
}
