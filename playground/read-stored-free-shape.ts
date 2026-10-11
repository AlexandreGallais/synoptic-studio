import { selectElement } from "./select-element";

import type { FreeShape } from "../src";

/**
 * Reads the value chosen in the shape selector.
 *
 * @kind procedure
 * @param document - page document
 * @returns `rectangle`, `polygon` or `free`; empty when the element is no selector
 * @see docs/backlog/stories/E01-F08-US-014-convert-and-move.md
 */
function readShapeValue(document: Document): string {
  const select = selectElement(document, "shape");

  return select instanceof HTMLSelectElement ? select.value : "";
}

/**
 * Reads the free shape the page keeps once a polygon is converted or a point typed (ADR-0033),
 * kept on the shape selector (`data-free-shape`).
 *
 * @kind procedure
 * @param document - page document
 * @returns the free shape kept, `undefined` when there is none or another shape is chosen
 * @see docs/backlog/stories/E01-F08-US-014-convert-and-move.md
 */
export function readStoredFreeShape(document: Document): FreeShape | undefined {
  const kept =
    readShapeValue(document) === "free"
      ? selectElement(document, "shape").dataset["freeShape"]
      : undefined;

  // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- written by writeStoredFreeShape only, from a FreeShape
  return kept === undefined ? undefined : (JSON.parse(kept) as FreeShape);
}
