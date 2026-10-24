import { positionAfterResize, straightBoxShift } from "../placement";

import { isPanelSize } from "./is-panel-size";
import { panelValueAfterEdit } from "./panel-value-after-edit";
import { shapeMovedOnAxis } from "./shape-moved-on-axis";
import { shapePanelValues } from "./shape-panel-values";
import { sizeWithPanelValue } from "./size-with-panel-value";

import type { PanelEdit } from "./panel-edit";
import type { Shape } from "../shape";

/**
 * A shape once W or H is typed for several shapes: its width or height set
 * (`sizeWithPanelValue`), a turned shape keeping its turned top-left corner
 * (`positionAfterResize`, US-019).
 *
 * @kind domain
 * @param shape - a valid shape of the model
 * @param side - `width` or `height`, the field typed
 * @param value - its new width or height (`panelValueAfterEdit`)
 * @returns the shape resized
 * @see docs/domain/interaction.md#_2-selector
 */
function shapeWithSide(shape: Shape, side: "height" | "width", value: number): Shape {
  const size = sizeWithPanelValue(shape, side, value);

  return { ...shape, ...size, ...positionAfterResize(shape, size) };
}

/**
 * A shape a tool makes once a value is typed for several shapes (F38), its new value given by
 * `panelValueAfterEdit` from its own (`shapePanelValues`): X or Y moves it so that its straight box
 * starts there, its position going by the difference (`straightBoxShift`, `shapeMovedOnAxis`), as for one shape
 * (ADR-0035); W or H sets its width or height (`shapeWithSide`).
 *
 * @kind domain
 * @param shape - a valid shape of the model
 * @param edit - the field, the value typed and the largest size
 * @returns the shape changed
 * @see docs/domain/interaction.md#_2-selector
 */
export function shapeAfterPanelEdit(shape: Shape, edit: PanelEdit): Shape {
  const { key } = edit;
  const own = shapePanelValues(shape)[key];
  const value = panelValueAfterEdit(own, edit);

  return isPanelSize(key)
    ? shapeWithSide(shape, key, value)
    : shapeMovedOnAxis(shape, key, straightBoxShift(own, value));
}
