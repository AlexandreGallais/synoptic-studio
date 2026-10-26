// lint-disable-next-line local/canonical-import-path -- the rule names the first folder under the common ancestor, playground/src, which has no barrel in the Angular app
import { writeShapeValues } from "../src/app/shared/utils/shape-fields";

import { loadPage } from "./load-page";

/**
 * Opens the playground on a regular hexagon in a 100 × 100 box, radius 0, centered, typed into the
 * panel before the app mounts the page, which draws it.
 *
 * @kind procedure
 */
export function openOnHexagon(): void {
  const page = loadPage("");

  writeShapeValues(document, {
    anchor: "mid mid",
    corners: 6,
    height: 100,
    radius: 0,
    shape: "polygon",
    width: 100,
  });

  page.changeDetectorRef.detectChanges();
}
