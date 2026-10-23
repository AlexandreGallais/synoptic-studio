import type { PanelEdit } from "./panel-edit";

/**
 * A shape's value once a value is typed for several shapes (F38): its own plus the value typed
 * with a sign, or the value typed without one, as Figma (`REF-FIGMA-ADJUST`); a width or height
 * capped at the interface's largest (Q20) and refused below 0.
 *
 * @kind domain
 * @param own - the shape's value in the field typed (`shapePanelValues`, `textPanelValues`)
 * @param edit - the field, the value typed and the largest size
 * @returns the shape's new value; `undefined` for a width or height below 0
 * @see docs/domain/interaction.md#_2-selector
 */
export function panelValueAfterEdit(own: number, edit: PanelEdit): number | undefined {
  return own + edit.maxSize;
}
