import type { PanelValues } from "editor";

/** Each value of several shapes, and the id of the field that shows it and sets it. */
export const SEVERAL_FIELDS: readonly (readonly [keyof PanelValues, string])[] = [
  ["x", "several-x"],
  ["y", "several-y"],
  ["width", "several-width"],
  ["height", "several-height"],
];
