import { parsePanelValue, radiiWithSelection, radiiWithTyped } from "../../src";
import { readNumber, readSelection, selectElement } from "../page";

import { MAX_SIZE } from "./max-size";
import { writeCappedSizes } from "./write-capped-sizes";

import type { TypedPanelValue } from "../../src";

/**
 * Reads a relative value left as typed in a radius field showing no number, a Mixed one, which
 * each vertex's own takes (`writeOwnValue`, AUD-046).
 *
 * @kind procedure
 * @param document - page document
 * @param id - `radius` or `point-radius`
 * @returns the value; undefined for a field showing a number, its value applied
 * @see docs/domain/interaction.md#_2-selector
 */
function readMixedTyped(document: Document, id: string): TypedPanelValue | undefined {
  const field = selectElement(document, id);
  const typed = parsePanelValue(field instanceof HTMLInputElement ? field.value : "");

  return typed?.isRelative === true ? typed : undefined;
}

/**
 * Applies the radius typed in the points' field to the selected vertices only, capped as every
 * size (F03.AC5, Q20); a relative value in a Mixed field added to each one's own
 * (`radiiWithTyped`); the radii are kept on the shape's field.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E01-F08-VAL-005-validate-node-editing.md
 */
function writePointRadius(document: Document): void {
  const shape = selectElement(document, "radius");
  const radii = (shape.dataset["radii"] ?? "").split(" ").map(Number);
  const { selected } = readSelection(document);
  const typed = readMixedTyped(document, "point-radius");

  writeCappedSizes(document);

  shape.dataset["radii"] = (
    typed === undefined
      ? radiiWithSelection(radii, selected, readNumber(document, "point-radius"))
      : radiiWithTyped(radii, selected, { maxSize: MAX_SIZE, typed })
  ).join(" ");
}

/**
 * Applies a radius typed in the shape's field to every vertex, the radii kept being dropped
 * (F03.AC2); a relative value in a Mixed field is added to each one's own instead, the radii kept
 * (`radiiWithTyped`, AUD-046).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E01-F03-US-011-radius-field-mixed.md
 */
function writeShapeRadius(document: Document): void {
  const field = selectElement(document, "radius");
  const { radii } = field.dataset;
  const typed = readMixedTyped(document, "radius");

  // Capped first, as every size typed (Q20).
  writeCappedSizes(document);

  if (typed !== undefined && radii !== undefined) {
    const kept = radii.split(" ").map(Number);
    const every = kept.map((radius, vertex) => vertex);

    field.dataset["radii"] = radiiWithTyped(kept, every, { maxSize: MAX_SIZE, typed }).join(" ");

    return;
  }

  delete field.dataset["radii"];
  field.setAttribute("placeholder", "");
}

/**
 * Applies a radius typed: the shape's field gives it to every vertex (`writeShapeRadius`), in
 * both modes; the points' field gives it to the selected vertices only (F03.AC5,
 * `writePointRadius`), each once applied, Enter or the field left (US-087).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E01-F03-US-013-points-list-selection.md
 */
export function listenToRadius(document: Document, update: () => void): void {
  selectElement(document, "radius").addEventListener("change", () => {
    writeShapeRadius(document);
  });

  selectElement(document, "point-radius").addEventListener("change", () => {
    writePointRadius(document);
    update();
  });
}
