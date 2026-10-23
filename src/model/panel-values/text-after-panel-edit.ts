import { positionAfterResize } from "../placement";

import { isPanelSize } from "./is-panel-size";
import { panelValueAfterEdit } from "./panel-value-after-edit";
import { shapeMovedOnAxis } from "./shape-moved-on-axis";
import { sizeWithPanelValue } from "./size-with-panel-value";
import { textPanelValues } from "./text-panel-values";

import type { PanelEdit } from "./panel-edit";
import type { Size } from "../../geometry";
import type { Text } from "../text";

/**
 * A text in a box once W or H is typed for several shapes: its box's width or height set
 * (`sizeWithPanelValue`), turned about its middle with its turned top-left corner kept
 * (`positionAfterResize`, ADR-0057).
 *
 * @kind domain
 * @param text - a text sitting in its box
 * @param side - `width` or `height`, the field typed
 * @param value - the box's new width or height (`panelValueAfterEdit`)
 * @returns the text with its box resized
 * @see docs/domain/interaction.md#_2-selector
 */
function textWithBoxSide(text: Text, side: "height" | "width", value: number): Text {
  const box = {
    angle: text.angle,
    height: text.boxHeight,
    width: text.boxWidth,
    x: text.x,
    y: text.y,
  };
  const size = sizeWithPanelValue(box, side, value);

  return {
    ...text,
    ...positionAfterResize(box, size),
    boxHeight: size.height,
    boxWidth: size.width,
  };
}

/**
 * A text once a value is typed for several shapes (F38), its new value given by
 * `panelValueAfterEdit` from its own (`textPanelValues`): X or Y moves it so that its straight box
 * starts there (`textStraightBox`, `shapeMovedOnAxis`); W or H sets the width or height of its box
 * when it sits in one (`textWithBoxSide`), while a text fitting its content keeps its size.
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
  const { key } = edit;

  if (isPanelSize(key) && !text.isInBox) {
    return text;
  }

  const own = textPanelValues(text, block)[key];
  const value = panelValueAfterEdit(own, edit);

  if (value === undefined) {
    return undefined;
  }

  return isPanelSize(key)
    ? textWithBoxSide(text, key, value)
    : shapeMovedOnAxis(text, key, value - own);
}
