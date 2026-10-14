/**
 * Angle at the centre of a regular polygon from its first vertex to another: n equal turns make
 * a whole turn, 2π, so each is 2π/n (`DERIV-stretched-polygon`); k of them are also the direction
 * of the k-th side, the sum of the turns before it.
 *
 * Formula: 2π · k / n, the product first.
 *
 * @kind math
 * @param step - number of sides from the first vertex
 * @param corners - number of corners n, ≥ 3
 * @returns the angle in radians
 * @see DERIV-stretched-polygon
 */
export function centralAngle(step: number, corners: number): number {
  return (2 * Math.PI * step) / corners;
}
