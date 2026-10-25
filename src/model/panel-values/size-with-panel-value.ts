import type { Size } from "../../geometry";

/**
 * A size with its width or its height set to the value typed for several shapes, the other kept.
 *
 * @kind domain
 * @param size - the width and height before
 * @param side - `width` or `height`, the field typed
 * @param value - the new width or height (`panelValueAfterEdit`)
 * @returns the size after
 * @see RULE-interaction-2-selector
 */
export function sizeWithPanelValue(size: Size, side: "height" | "width", value: number): Size {
  return side === "width"
    ? { height: size.height, width: value }
    : { height: value, width: size.width };
}
