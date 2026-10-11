import { selectElement } from "./select-element";

/**
 * Reads the mode chosen in the tool bar: Shape, or Points to show and select the vertices
 * (F03.AC4, `REF-INKSCAPE-NODE-TOOL`).
 *
 * @kind procedure
 * @param document - page document
 * @returns `points` when the Points button is pressed, `shape` otherwise
 * @see docs/backlog/stories/E01-F03-US-012-points-mode.md
 */
export function readMode(document: Document): "points" | "shape" {
  return selectElement(document, "mode-points").getAttribute("aria-pressed") === "true"
    ? "points"
    : "shape";
}
