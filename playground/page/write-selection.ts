import { selectElement } from "./select-element";

import type { VertexSelection } from "../../src";

/**
 * Keeps the vertices selected in Points mode on the list of points (`data-selected`,
 * `data-start`), read back by `readSelection`.
 *
 * @kind procedure
 * @param document - page document
 * @param selection - selection to keep
 * @see docs/backlog/stories/E01-F03-US-013-points-list-selection.md
 */
export function writeSelection(document: Document, selection: VertexSelection): void {
  const list = selectElement(document, "points");

  list.dataset["selected"] = selection.selected.join(" ");

  if (selection.start === undefined) {
    delete list.dataset["start"];
  } else {
    list.dataset["start"] = String(selection.start);
  }
}
