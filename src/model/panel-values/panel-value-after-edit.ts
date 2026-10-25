import { clamp } from "../../math";

import { isPanelSize } from "./is-panel-size";
import { wholeValueWithTyped } from "./whole-value-with-typed";

import type { PanelEdit } from "./panel-edit";

/**
 * A shape's value once a value is typed in X, Y, W or H (F38, AUD-046): its own plus the value
 * typed with a sign, or the value typed without one, as Figma (`REF-FIGMA-ADJUST`), rounded when
 * a fraction is typed (`wholeValueWithTyped`); a width or height taken within 0 and the
 * interface's largest (Q20), so that `-50` on a width of 40 gives 0 and 200 000 gives 100 000.
 *
 * @kind domain
 * @param own - the shape's value in the field typed (`shapePanelValues`, `textPanelValues`)
 * @param edit - the field, the value typed and the largest size
 * @returns the shape's new value
 * @see RULE-interaction-2-selector
 */
export function panelValueAfterEdit(own: number, edit: PanelEdit): number {
  const { key, maxSize, typed } = edit;
  const value = wholeValueWithTyped(own, typed);

  return isPanelSize(key) ? clamp(value, 0, maxSize) : value;
}
