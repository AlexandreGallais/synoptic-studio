import { selectElement } from "./select-element";

import type { VertexSelection } from "editor";

/**
 * Reads the vertices selected in Points mode, kept on the list of points (`data-selected`,
 * `data-start`).
 *
 * @kind procedure
 * @param document - page document
 * @returns the selection, empty when none was kept
 */
export function readSelection(document: Document): VertexSelection {
  const { selected = "", start } = selectElement(document, "points").dataset;

  return {
    selected: selected === "" ? [] : selected.split(" ").map(Number),
    start: start === undefined ? undefined : Number(start),
  };
}
