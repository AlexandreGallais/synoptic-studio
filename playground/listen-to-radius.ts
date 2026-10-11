import { radiiWithSelection } from "../src";

import { readMode } from "./read-mode";
import { readSelection } from "./read-selection";
import { selectElement } from "./select-element";
import { writeCappedSizes } from "./write-capped-sizes";

/**
 * Applies a radius typed: in Shape mode it goes to every vertex, the radii kept being dropped
 * (F03.AC2); in Points mode it goes to the selected vertices only (F03.AC5).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E01-F03-US-013-points-list-selection.md
 */
export function listenToRadius(document: Document): void {
  const field = selectElement(document, "radius");

  field.addEventListener("input", () => {
    // Capped first, as every size typed (Q20).
    writeCappedSizes(document);

    const { radii = "" } = field.dataset;
    const typed = field instanceof HTMLInputElement ? field.valueAsNumber : NaN;
    const kept = radii.split(" ").map(Number);
    const { selected } = readSelection(document);

    field.dataset["radii"] = radiiWithSelection(kept, selected, typed).join(" ");

    if (readMode(document) !== "shape") {
      return;
    }

    delete field.dataset["radii"];
    field.setAttribute("placeholder", "");
  });
}
