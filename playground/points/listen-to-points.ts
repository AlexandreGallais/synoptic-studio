import { selectionAfterHandleClick, selectionAfterRowClick } from "../../src";
import { listenToHandleFields } from "../handles";
import { readSelection, selectElement, writeSelection } from "../page";

import { listenToPointFields } from "./listen-to-point-fields";

import type { ClickKeys } from "../../src";

/** How a click changes the selection: on a handle or on a row of the list. */
type NextSelection = typeof selectionAfterRowClick;

/**
 * Reads the vertex a click hit: the `data-vertex` of the handle or the row clicked, counted
 * from 1 on the page, from 0 in the selection.
 *
 * @kind procedure
 * @param event - mouse click on the drawing or the list
 * @returns the vertex number from 0, `NaN` when the click hit no vertex
 * @see docs/backlog/stories/E01-F03-US-013-points-list-selection.md
 */
function readClickedVertex(event: Event): number {
  const target =
    event.target instanceof Element
      ? event.target.closest<HTMLElement>("[data-vertex]")
      : undefined;

  return Number(target?.dataset["vertex"]) - 1;
}

/**
 * Reads the modifier keys of a click.
 *
 * @kind procedure
 * @param event - mouse click on a handle or a row
 * @returns Shift, and Ctrl or Cmd (macOS) as the key that adds or removes a vertex
 * @see REF-APPLE-SELECT
 */
function readClickKeys(event: MouseEvent): ClickKeys {
  return { shift: event.shiftKey, toggle: event.ctrlKey || event.metaKey };
}

/**
 * Creates the click listener of the drawing or of the list: the vertex hit changes the
 * selection, then the page is redrawn; a click beside the vertices changes nothing.
 *
 * @kind procedure
 * @param document - page document
 * @param next - how the click changes the selection
 * @param update - redraws the playground
 * @returns a function to register for `click`
 * @see docs/backlog/stories/E01-F03-US-013-points-list-selection.md
 */
function createClickListener(
  document: Document,
  next: NextSelection,
  update: () => void,
): (event: MouseEvent) => void {
  return (event) => {
    const vertex = readClickedVertex(event);

    if (Number.isNaN(vertex)) {
      return;
    }

    writeSelection(document, next(readSelection(document), vertex, readClickKeys(event)));
    update();
  };
}

/**
 * Selects vertices when their handle or their row is clicked in Points mode, with Shift and
 * Ctrl (Cmd) as in `docs/domain/interaction.md` §3, then calls back (F03.AC5).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E01-F03-US-013-points-list-selection.md
 */
export function listenToPoints(document: Document, update: () => void): void {
  selectElement(document, "canvas").addEventListener(
    "click",
    createClickListener(document, selectionAfterHandleClick, update),
  );

  selectElement(document, "points").addEventListener(
    "click",
    createClickListener(document, selectionAfterRowClick, update),
  );

  listenToPointFields(document, update);
  listenToHandleFields(document, update);
}
