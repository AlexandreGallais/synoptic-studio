import { EPSILON, ceiling } from "../../math";

import type { Size } from "../../geometry";
import type { Text } from "../text";

/**
 * The smallest box of integer sides that holds a text's measured block, so that a text that fits
 * its content keeps its lines and its place when it is put in a box: each side rounded up
 * (`ceiling`), a side within a rounding error (`EPSILON`) of an integer kept, since a box holds a
 * line within that error (`DERIV-text-box`, step 11).
 *
 * Formula: W = ⌈w − ε⌉, H = ⌈h − ε⌉.
 *
 * @kind domain
 * @param block - the measured block, as wide as its widest line and its lines tall
 * @returns the box's width and height, integers
 * @see DERIV-text-box
 */
export function boxAroundBlock(block: Size): Pick<Text, "boxHeight" | "boxWidth"> {
  return { boxHeight: ceiling(block.height - EPSILON), boxWidth: ceiling(block.width - EPSILON) };
}
