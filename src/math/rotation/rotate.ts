import type { Vector } from "../vector";

/**
 * Vector turned by the angle whose cosine and sine are a direction's components: the rotation
 * matrix applied to it, clockwise on screen where y points down.
 *
 * Formula: (vₓ cos θ − v_y sin θ, vₓ sin θ + v_y cos θ), with (cos θ, sin θ) = d.
 *
 * @kind math
 * @param vector - vector to turn
 * @param direction - unit vector (cos θ, sin θ) of the angle
 * @returns the turned vector
 * @see REF-MATHWORLD-ROTATION-MATRIX
 */
export function rotate(vector: Vector, direction: Vector): Vector {
  return {
    x: vector.x * direction.x - vector.y * direction.y,
    y: vector.x * direction.y + vector.y * direction.x,
  };
}
