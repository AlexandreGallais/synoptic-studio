import { selectElement } from "./select-element";

import type { VertexSelection } from "../../src";

/**
 * Reads the vertices selected in Points mode, kept on the list of points (`data-selected`,
 * `data-start`).
 *
 * @kind procedure
 * @param document - page document
 * @returns the selection, empty when none was kept
 * @see docs/backlog/stories/E01-F03-US-013-points-list-selection.md
 */
export function readSelection(document: Document): VertexSelection {
  const { selected = "", start } = selectElement(document, "points").dataset;

  return {
    selected: selected === "" ? [] : selected.split(" ").map(Number),
    start: start === undefined ? undefined : Number(start),
  };
}
