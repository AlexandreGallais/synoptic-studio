import { freeShapeRadii, isValidFreeShape } from "../../src";
import { saveKindSettings } from "../kind-settings";
import { readShapeKind, selectElement, writeInputValue } from "../page";

import type { FreeShape } from "../../src";

/**
 * Chooses "Free shape" in the shape selector, shown from now on.
 *
 * @kind procedure
 * @param document - page document
 * @see US-014
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
 * Keeps a valid free shape on the page (ADR-0033), leaving the one kept when it is not: stored
 * on the shape selector, which shows "Free shape", its width and height written in their inputs
 * and its radii kept on the radius input; a rectangle or a polygon becoming free keeps its
 * settings first, as the last shape of its kind (SP-026).
 *
 * @kind procedure
 * @param document - page document
 * @param shape - free shape to keep
 * @see US-014
 */
export function writeStoredFreeShape(document: Document, shape: FreeShape): void {
  // A move that leaves no width or no height is no shape: the one kept stays (AUD-005).
  if (!isValidFreeShape(shape)) {
    return;
  }

  const select = selectElement(document, "shape");

  saveKindSettings(document, readShapeKind(document));
  select.dataset["freeShape"] = JSON.stringify(shape);
  selectFreeShape(document);
  writeInputValue(document, "width", shape.width);
  writeInputValue(document, "height", shape.height);

  selectElement(document, "radius").dataset["radii"] = freeShapeRadii(shape).join(" ");
}
