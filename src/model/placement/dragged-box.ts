import { nearestInteger } from "../../math";

import type { Size } from "../../geometry";
import type { Point } from "../../math";

/** A drag with a shape's tool: where the pointer was pressed and released, and Shift. */
export type ShapeDrag = {
  /** The point pressed, in the coordinates of the SVG, possibly fractional. */
  readonly press: Point;
  /** The point the pointer is on, or released at, possibly fractional. */
  readonly release: Point;
  /** Whether the box stays square, Shift held for every tool but text. */
  readonly isSquare: boolean;
};

/**
 * Start of a box's side laid from the press toward the release: the press less the side when the
 * release lies before it, the press otherwise.
 *
 * @kind domain
 * @param press - the coordinate pressed, an integer
 * @param release - the coordinate released, an integer
 * @param side - the side's length, an integer ≥ 0
 * @returns the smaller end of the side
 * @see RULE-interaction-1-tools
 */
function sideStart(press: number, release: number, side: number): number {
  return release < press ? press - side : press;
}

/**
 * Box of a shape drawn by a drag: the press and the release each rounded to the nearest
 * integer (`nearestInteger`), the box running from the smaller to the larger of each coordinate,
 * whatever the direction dragged; square, its side is the larger of the two distances, laid from
 * the press toward the release, down or right when the release is level with the press. A release
 * on the press's integer point gives a box 0 × 0 at that point: a click.
 *
 * Formula: a = round(press), b = round(release); w = |bx − ax|, h = |by − ay|, both
 * max(w, h) when square; x = ax − w when bx < ax, else ax; likewise y.
 *
 * @kind domain
 * @param drag - the point pressed, the point released and whether the box stays square
 * @returns the top-left of the box, its width and its height, integers
 * @see RULE-interaction-1-tools
 */
export function draggedBox(drag: ShapeDrag): Point & Size {
  const press = { x: nearestInteger(drag.press.x), y: nearestInteger(drag.press.y) };
  const release = { x: nearestInteger(drag.release.x), y: nearestInteger(drag.release.y) };
  const width = Math.abs(release.x - press.x);
  const height = Math.abs(release.y - press.y);
  const side = Math.max(width, height);
  const size = drag.isSquare ? { height: side, width: side } : { height, width };

  return {
    ...size,
    x: sideStart(press.x, release.x, size.width),
    y: sideStart(press.y, release.y, size.height),
  };
}
