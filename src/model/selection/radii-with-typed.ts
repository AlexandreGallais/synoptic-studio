import { clamp } from "../../math";
import { wholeValueWithTyped } from "../panel-values";

import type { PanelEdit } from "../panel-values";

/**
 * Radii after a relative value is typed in a radius field showing Mixed, the radii differing:
 * each vertex the field speaks for adds it to its own radius, as each of several
 * shapes adds a value to its own, rounded when a fraction is typed (`wholeValueWithTyped`)
 * and taken within 0 and the interface's largest size (Q20); the others keep theirs. Radii 10, 4
 * and 20 and `-5` give 5, 0 and 15.
 *
 * @kind domain
 * @param radii - requested radius of each vertex, in contour order
 * @param selected - numbers of the vertices the field speaks for, from 0
 * @param edit - the value typed, relative, and the largest size
 * @returns the new radii, as many as before
 * @see RULE-shapes-2-corner-radius
 */
export function radiiWithTyped(
  radii: readonly number[],
  selected: readonly number[],
  edit: Omit<PanelEdit, "key">,
): readonly number[] {
  const { maxSize, typed } = edit;

  return radii.map((kept, index) =>
    selected.includes(index) ? clamp(wholeValueWithTyped(kept, typed), 0, maxSize) : kept,
  );
}
