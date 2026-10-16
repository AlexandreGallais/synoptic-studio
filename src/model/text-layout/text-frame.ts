import type { Size } from "../../geometry";
import type { TurnedBox } from "../placement";
import type { Text } from "../text";

/**
 * The rectangle a text is laid out in, at its position: its box when it sits in one, its lines
 * aligned across the box's width and its block placed down the box's height; otherwise its
 * measured block, as in F05 (`DERIV-text-box`, step 8). The text turns about its middle
 * (`DERIV-turned-shape`, step 4).
 *
 * @kind domain
 * @param text - its position, its box and whether it sits in it
 * @param block - its measured block (`textBlock`)
 * @returns the rectangle's position, width and height, in user units
 * @see DERIV-text-box
 */
export function textFrame(text: Text, block: Size): Omit<TurnedBox, "angle"> {
  return text.isInBox
    ? { height: text.boxHeight, width: text.boxWidth, x: text.x, y: text.y }
    : { ...block, x: text.x, y: text.y };
}
