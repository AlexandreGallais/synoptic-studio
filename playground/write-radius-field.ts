import { commonRadius } from "../src";

import { selectElement } from "./select-element";

/**
 * Shows the radii of a valid shape in its radius field: their common value, or an empty field
 * reading "Mixed" when they differ, the radii then kept on the input (`data-radii`) as typed
 * values cannot hold them (F03.AC2, `REF-FIGMA-CR`).
 *
 * @kind procedure
 * @param document - page document
 * @param radii - requested radius of each vertex, valid
 * @see docs/backlog/stories/E01-F03-US-011-radius-field-mixed.md
 */
export function writeRadiusField(document: Document, radii: readonly number[]): void {
  const field = selectElement(document, "radius");

  if (!(field instanceof HTMLInputElement)) {
    return;
  }

  const common = commonRadius(radii);

  if (common === undefined) {
    field.dataset["radii"] = radii.join(" ");
    field.value = "";
    field.placeholder = "Mixed";
  } else {
    delete field.dataset["radii"];
    field.value = String(common);
    field.placeholder = "";
  }
}
