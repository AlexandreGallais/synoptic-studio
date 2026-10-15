/**
 * How high a point sits above the bottom edge of the plane, its `y` measured down from the top
 * edge, as the page measures it: the plane's height less `y`. The same subtraction gives `y` back
 * from the height, so it serves both ways (`DERIV-color-spaces`, step 10: `L or V = 100 · (h − y)
 * / h`, `y = h − h · L / 100`). On a plane 100 tall, the point at `y = 25` is 75 high.
 *
 * Formula: h − y.
 *
 * @kind domain
 * @param y - the distance down from the top edge, or the height above the bottom edge
 * @param height - the plane's height
 * @returns the other one: the height above the bottom edge, or the distance down from the top
 * @see DERIV-color-spaces
 */
export function heightFromBottom(y: number, height: number): number {
  return height - y;
}
