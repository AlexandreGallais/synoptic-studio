import type { EditedField } from "./edited-field";

/**
 * Each value of the shape the panel edits, alone, and the id of the field that shows it and sets
 * it, as several shapes' (`SEVERAL_FIELDS`): X and Y for a shape and a text alike, the
 * shape's Width and Height, then a text's box's.
 */
export const EDITED_FIELDS: readonly EditedField[] = [
  ["x", "shape-x"],
  ["y", "shape-y"],
  ["width", "width"],
  ["height", "height"],
  ["width", "text-box-width"],
  ["height", "text-box-height"],
];
