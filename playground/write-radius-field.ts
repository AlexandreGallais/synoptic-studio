import { commonRadius } from "../src";

import { selectElement } from "./select-element";

/**
 * Hint of the radius field when it holds no value: no vertex to speak for, or radii that differ.
 *
 * @kind procedure
 * @param common - radius shared by the vertices shown, `undefined` when there is none
 * @param count - number of vertices the field speaks for
 * @returns `Select points`, `Mixed`, or nothing when the field shows a value
 * @see docs/backlog/stories/E01-F03-US-011-radius-field-mixed.md
 */
function createPlaceholder(common: number | undefined, count: number): string {
  if (count === 0) {
    return "Select points";
  }

  return common === undefined ? "Mixed" : "";
}

/**
 * Shows in the radius field the radii it speaks for (every vertex in Shape mode, the selected
 * ones in Points mode): their common value, or an empty field reading "Mixed" when they differ
 * (F03.AC2, F03.AC5, `REF-FIGMA-CR`); disabled when no vertex is selected. Every radius is kept
 * on the input (`data-radii`), as one typed value cannot hold them all.
 *
 * @kind procedure
 * @param document - page document
 * @param radii - requested radius of each vertex, valid
 * @param shown - numbers of the vertices the field speaks for, from 0
 * @see docs/backlog/stories/E01-F03-US-011-radius-field-mixed.md
 */
export function writeRadiusField(
  document: Document,
  radii: readonly number[],
  shown: readonly number[],
): void {
  const field = selectElement(document, "radius");

  if (!(field instanceof HTMLInputElement)) {
    return;
  }

  const common = commonRadius(radii.filter((radius, vertex) => shown.includes(vertex)));

  field.dataset["radii"] = radii.join(" ");
  field.disabled = shown.length === 0;
  field.value = common === undefined ? "" : String(common);
  field.placeholder = createPlaceholder(common, shown.length);
}
