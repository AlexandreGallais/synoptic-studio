import type { PanelValues } from "../../src";

/** Each value of several shapes, and the id of the field that shows it and sets it (F38). */
export const SEVERAL_FIELDS: readonly (readonly [keyof PanelValues, string])[] = [
  ["x", "several-x"],
  ["y", "several-y"],
  ["width", "several-width"],
  ["height", "several-height"],
];
