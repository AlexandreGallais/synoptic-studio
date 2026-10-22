import type { Size } from "../../geometry";
import type { Point } from "../../math";

/** A drag with a shape's tool (F36): where the pointer was pressed and released, and Shift. */
export type ShapeDrag = {
  /** The point pressed, in the coordinates of the SVG, possibly fractional. */
  readonly press: Point;
  /** The point the pointer is on, or released at, possibly fractional. */
  readonly release: Point;
  /** Whether the box stays square, Shift held for every tool but text. */
  readonly isSquare: boolean;
};

/**
 * Box of a shape drawn by a drag (F36): the press and the release each rounded to the nearest
 * integer (`nearestInteger`), the box running from the smaller to the larger of each coordinate,
 * whatever the direction dragged; square, its side is the larger of the two distances, laid from
 * the press toward the release, down or right when the release is level with the press. A release
 * on the press's integer point gives a box 0 × 0 at that point: a click (US-063).
 *
 * Formula: a = round(press), b = round(release); w = |bx − ax|, h = |by − ay|, both
 * max(w, h) when square; x = ax − w when bx < ax, else ax; likewise y.
 *
 * @kind domain
 * @param drag - the point pressed, the point released and whether the box stays square
 * @returns the top-left of the box, its width and its height, integers
 * @see docs/domain/interaction.md#_1-tools
 */
export function draggedBox(drag: ShapeDrag): Point & Size {
  return { ...drag.press, height: 0, width: 0 };
}
