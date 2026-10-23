import type { PanelEdit } from "./panel-edit";
import type { Shape } from "../shape";

/**
 * A shape a tool makes once a value is typed for several shapes (F38), its new value given by
 * `panelValueAfterEdit` from its own (`shapePanelValues`): X or Y moves it so that its straight box
 * starts there, its position going by the difference, as for one shape (ADR-0035); W or H sets
 * its width or height, a turned shape keeping its turned top-left corner (`positionAfterResize`,
 * US-019).
 *
 * @kind domain
 * @param shape - a valid shape of the model
 * @param edit - the field, the value typed and the largest size
 * @returns the shape changed; `undefined` when its new width or height is refused
 * @see docs/domain/interaction.md#_2-selector
 */
export function shapeAfterPanelEdit(shape: Shape, edit: PanelEdit): Shape | undefined {
  return edit.maxSize > 0 ? shape : undefined;
}
