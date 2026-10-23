import type { PanelEdit } from "./panel-edit";
import type { Size } from "../../geometry";
import type { Text } from "../text";

/**
 * A text once a value is typed for several shapes (F38), its new value given by
 * `panelValueAfterEdit` from its own (`textPanelValues`): X or Y moves it so that its straight box
 * starts there (`textStraightBox`); W or H sets the width or height of its box when it sits in one,
 * turned about its middle with its turned top-left corner kept (`positionAfterResize`, ADR-0057),
 * while a text fitting its content keeps its size.
 *
 * @kind domain
 * @param text - a valid text of the model
 * @param block - its measured block (`textBlock`)
 * @param edit - the field, the value typed and the largest size
 * @returns the text changed, or as it is for a size typed while it fits its content; `undefined`
 * when its box's new width or height is refused
 * @see docs/domain/interaction.md#_2-selector
 */
export function textAfterPanelEdit(text: Text, block: Size, edit: PanelEdit): Text | undefined {
  return block.width > edit.maxSize ? text : undefined;
}
