/**
 * Distance from a corner vertex to the tangent points of its fillet arc.
 *
 * Formula: s = r · tan(|τ| / 2) = r / tan(θ / 2), τ = turning angle, θ = interior angle.
 *
 * @kind geometry
 * @param radius - fillet radius, >= 0
 * @param turningAngle - signed turning angle at the vertex in radians, in ]−π, π[
 * @returns setback along each adjacent edge, 0 for aligned vertices
 * @see DERIV-fillet-setback
 */
export function filletSetback(radius: number, turningAngle: number): number {
  return radius * Math.tan(Math.abs(turningAngle) / 2);
}
