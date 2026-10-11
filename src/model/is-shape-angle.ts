/** Degrees in a whole turn. */
const TURN = 360;

/**
 * Whether an angle can be stored by a shape: an integer from 0 to 359 (ADR-0035).
 *
 * An angle typed outside that range is first kept from 0 to 359 (`degreesInTurn`); the model
 * stores only the kept one, so that one drawing has one angle.
 *
 * @kind domain
 * @param angle - number of degrees
 * @returns `true` when it is a safe integer from 0 to 359
 * @see docs/domain/shapes.md#rotation-of-shapes
 */
export function isShapeAngle(angle: number): boolean {
  return Number.isSafeInteger(angle) && angle >= 0 && angle < TURN;
}
