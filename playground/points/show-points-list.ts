import { cyclicItem, drawnRadii, formatSvgNumber, isReducedRadius } from "../../src";
import { readMode, selectElement } from "../page";

import type { Corner, VertexSelection } from "../../src";
import type { ShapeReading } from "../shape";

/**
 * Text of a row of the list of points: its number from 1, x and y with at most 5 decimals
 * (Q10), the requested radius, and the effective one when it was reduced (Q8).
 *
 * @kind procedure
 * @param corner - vertex and its requested radius
 * @param vertex - number of the vertex, from 0
 * @param effective - radius drawn at this vertex, after clamping
 * @returns a line such as `1 · x 25 · y 6.69873 · radius 10`
 * @see RULE-interaction-3-node-editing
 */
function createRowText(corner: Corner, vertex: number, effective: number): string {
  const { point, radius } = corner;
  const reduced = isReducedRadius(radius, effective)
    ? ` (effective ${formatSvgNumber(effective)})`
    : "";
  const place = `x ${formatSvgNumber(point.x)} · y ${formatSvgNumber(point.y)}`;

  return `${String(vertex + 1)} · ${place} · radius ${String(radius)}${reduced}`;
}

/**
 * Shows the list of the points of the shape in Points mode, in contour order: one button per
 * vertex, pressed when the vertex is selected (F03.AC4, F03.AC5); hidden in Shape mode.
 *
 * @kind procedure
 * @param document - page document
 * @param reading - valid shape
 * @param selection - vertices selected
 */
export function showPointsList(
  document: Document,
  reading: ShapeReading,
  selection: VertexSelection,
): void {
  const list = selectElement(document, "points");
  // The radii drawn: a point in handles mode keeps no fillet; beside a curve, ADR-0047.
  const effective = drawnRadii(reading.corners);
  const rows = reading.corners.map((corner, vertex) => {
    const row = document.createElement("li");
    const button = document.createElement("button");

    button.type = "button";
    button.dataset["vertex"] = String(vertex + 1);
    button.setAttribute("aria-pressed", String(selection.selected.includes(vertex)));
    button.textContent = createRowText(corner, vertex, cyclicItem(effective, vertex, 0));
    row.append(button);

    return row;
  });

  list.hidden = readMode(document) !== "points";
  list.replaceChildren(...rows);
}
