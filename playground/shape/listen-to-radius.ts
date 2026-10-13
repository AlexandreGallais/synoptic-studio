import { radiiWithSelection } from "../../src";
import { readNumber, readSelection, selectElement } from "../page";

import { writeCappedSizes } from "./write-capped-sizes";

/**
 * Applies the radius typed in the points' field to the selected vertices only, capped as every
 * size (F03.AC5, Q20); the radii are kept on the shape's field.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E01-F08-VAL-005-validate-node-editing.md
 */
function writePointRadius(document: Document): void {
  const shape = selectElement(document, "radius");
  const { radii = "" } = shape.dataset;

  writeCappedSizes(document);

  const typed = readNumber(document, "point-radius");
  const { selected } = readSelection(document);

  shape.dataset["radii"] = radiiWithSelection(radii.split(" ").map(Number), selected, typed).join(
    " ",
  );
}

/**
 * Applies a radius typed: the shape's field gives it to every vertex, the radii kept being
 * dropped (F03.AC2), in both modes; the points' field gives it to the selected vertices only
 * (F03.AC5). Neither field is moved nor rewritten while typed in, so typing goes on (VAL-005).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E01-F03-US-013-points-list-selection.md
 */
export function listenToRadius(document: Document, update: () => void): void {
  const field = selectElement(document, "radius");

  field.addEventListener("input", () => {
    // Capped first, as every size typed (Q20).
    writeCappedSizes(document);
    delete field.dataset["radii"];
    field.setAttribute("placeholder", "");
  });

  selectElement(document, "point-radius").addEventListener("input", () => {
    writePointRadius(document);
    update();
  });
}
