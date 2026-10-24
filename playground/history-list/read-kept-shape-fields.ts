import type { KeptShape } from "../kept-shapes";

/**
 * The field a shape's corners are read under, by the kind the panel shows it as, so that a step
 * changing them alone takes the label of the panel's field that changes them (`fieldLabel`): a
 * rectangle's corners only by its Radius, a free shape's as its Points; a polygon's `corners`, its
 * number of corners, kept.
 */
const CORNER_FIELDS: ReadonlyMap<string, string> = new Map([
  ["free", "points"],
  ["rectangle", "radius"],
]);

/**
 * Reads the fields of a shape the page keeps that a step's name is read from (US-077): the
 * shape's own, its corners under the field the panel changes them by (`CORNER_FIELDS`), and
 * `kind`, the kind the panel shows it as, so that Convert to free shape, which may keep the shape
 * and change its kind alone, is named `Convert` (`stepName`).
 *
 * @kind procedure
 * @param kept - a shape as the page keeps it
 * @returns its shape's fields and its kind
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function readKeptShapeFields(kept: KeptShape): Readonly<Record<string, unknown>> {
  const field = CORNER_FIELDS.get(kept.kind);

  if (field === undefined || !("corners" in kept.shape)) {
    return { ...kept.shape, kind: kept.kind };
  }

  const { corners, ...fields } = kept.shape;

  return { ...fields, [field]: corners, kind: kept.kind };
}
