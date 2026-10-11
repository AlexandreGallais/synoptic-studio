import { formatSvgNumber } from "../src";

import { selectElement } from "./select-element";

import type { FreeShape } from "../src";

/**
 * Writes a size into its input as text, fractions included (a converted polygon, Q27).
 *
 * @kind procedure
 * @param document - page document
 * @param id - `width` or `height`
 * @param size - the size to show
 * @see docs/backlog/stories/E01-F08-US-014-convert-and-move.md
 */
function writeSizeText(document: Document, id: string, size: number): void {
  const input = selectElement(document, id);

  if (input instanceof HTMLInputElement) {
    input.value = formatSvgNumber(size);
  }
}

/**
 * Chooses "Free shape" in the shape selector, shown from now on.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E01-F08-US-014-convert-and-move.md
 */
function selectFreeShape(document: Document): void {
  const option = selectElement(document, "shape").querySelector('option[value="free"]');

  if (!(option instanceof HTMLOptionElement)) {
    return;
  }

  option.hidden = false;
  option.selected = true;
}

/**
 * Keeps a free shape on the page (ADR-0033): stored on the shape selector, which shows "Free
 * shape", its width and height written in their inputs and its radii kept on the radius input.
 *
 * @kind procedure
 * @param document - page document
 * @param shape - free shape to keep
 * @see docs/backlog/stories/E01-F08-US-014-convert-and-move.md
 */
export function writeStoredFreeShape(document: Document, shape: FreeShape): void {
  const select = selectElement(document, "shape");

  select.dataset["freeShape"] = JSON.stringify(shape);
  selectFreeShape(document);
  writeSizeText(document, "width", shape.width);
  writeSizeText(document, "height", shape.height);

  selectElement(document, "radius").dataset["radii"] = shape.corners
    .map((corner) => corner.radius)
    .join(" ");
}
