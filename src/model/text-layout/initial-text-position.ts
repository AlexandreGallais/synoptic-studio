import type { TextDirection } from "../text";

/**
 * The abscissa a line's `<tspan>` is written at, its initial current text position: with the
 * initial text anchor, `start`, the left side of left-to-right text is drawn there and the right
 * side of right-to-left text (`REF-SVG2-TEXT`, §11.10.1.1), so no `text-anchor` is written.
 *
 * Formula: left for LTR; left + w for RTL.
 *
 * @kind domain
 * @param left - the line's left edge
 * @param width - the line's width
 * @param direction - left to right or right to left
 * @returns the `x` of the line
 * @see REF-SVG2-TEXT
 */
export function initialTextPosition(left: number, width: number, direction: TextDirection): number {
  return direction === "rtl" ? left + width : left;
}
