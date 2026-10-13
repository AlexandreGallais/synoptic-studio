import { commonRadius } from "../../src";
import { selectElement } from "../page";

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

/** The radii of a shape and the vertices a radius field speaks for. */
export type RadiiShown = {
  /** Requested radius of each vertex, valid. */
  readonly all: readonly number[];
  /** Numbers of the vertices the field speaks for, from 0. */
  readonly shown: readonly number[];
};

/**
 * Shows in a radius field the radii it speaks for (every vertex for the shape's field, the
 * selected ones for the points' field): their common value, or an empty field reading "Mixed"
 * when they differ (F03.AC2, F03.AC5, `REF-FIGMA-CR`); disabled when it speaks for no vertex.
 * Every radius is kept on the shape's field (`data-radii`), as one typed value cannot hold them
 * all.
 *
 * @kind procedure
 * @param document - page document
 * @param id - `radius` (the shape's field) or `point-radius` (the points' field)
 * @param radii - requested radius of each vertex, and the vertices the field speaks for
 * @see docs/backlog/stories/E01-F03-US-011-radius-field-mixed.md
 */
export function writeRadiusField(document: Document, id: string, radii: RadiiShown): void {
  const field = selectElement(document, id);

  if (!(field instanceof HTMLInputElement)) {
    return;
  }

  const { all, shown } = radii;
  const common = commonRadius(all.filter((radius, vertex) => shown.includes(vertex)));

  selectElement(document, "radius").dataset["radii"] = all.join(" ");
  field.disabled = shown.length === 0;
  field.value = common === undefined ? "" : String(common);
  field.placeholder = createPlaceholder(common, shown.length);
}
