import { textFrame, textStraightBox } from "../text-layout";

import type { PanelValues } from "./panel-values";
import type { Size } from "../../geometry";
import type { Text } from "../text";

/**
 * The position and size the panel shows for a text (F38), by its frame, its block or its box when
 * it sits in one, as under the pointer: the x and y of its straight box (`textStraightBox`), its
 * frame's width and height (`textFrame`).
 *
 * @kind domain
 * @param text - its position, its box, whether it sits in it, and its angle
 * @param block - its measured block (`textBlock`)
 * @returns its values
 * @see RULE-interaction-2-selector
 */
export function textPanelValues(text: Text, block: Size): PanelValues {
  const { height, width } = textFrame(text, block);
  const box = textStraightBox(text, block);

  return { height, width, x: box.minX, y: box.minY };
}
