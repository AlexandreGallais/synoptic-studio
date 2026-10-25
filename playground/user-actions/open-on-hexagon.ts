import { mountPlayground } from "../mount";
import { writeShapeValues } from "../shape-fields";

import { loadPage } from "./load-page";

/**
 * Opens the playground on a regular hexagon in a 100 × 100 box, radius 0, centered, typed into the
 * panel before the mount draws it.
 *
 * @kind procedure
 */
export function openOnHexagon(): void {
  loadPage("");

  writeShapeValues(document, {
    anchor: "mid mid",
    corners: 6,
    height: 100,
    radius: 0,
    shape: "polygon",
    width: 100,
  });

  mountPlayground(document);
}
