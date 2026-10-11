import { isValidFreeShape } from "../src";

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
 * Reads back the free shape kept as text, checked again: a value lost on the way (`NaN` written
 * as `null`) makes it no shape.
 *
 * @kind procedure
 * @param kept - text written by `writeStoredFreeShape`
 * @returns the free shape, `undefined` when it is not valid
 * @see docs/backlog/stories/E01-F08-US-014-convert-and-move.md
 */
function readKeptText(kept: string): FreeShape | undefined {
  // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- written by writeStoredFreeShape only, from a FreeShape; checked just below
  const shape = JSON.parse(kept) as FreeShape;

  return isValidFreeShape(shape) ? shape : undefined;
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
  const { freeShape } = selectElement(document, "shape").dataset;

  return freeShape !== undefined && readShapeValue(document) === "free"
    ? readKeptText(freeShape)
    : undefined;
}
