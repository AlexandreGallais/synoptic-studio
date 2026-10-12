/**
 * Whether a size or a radius can enter the model: an integer ≥ 0 (ADR-0003, Q15).
 *
 * 0 is allowed (a degenerate shape, useful in animations); a negative, fractional or unsafe
 * value is rejected. Shared by every shape of the model.
 *
 * @kind domain
 * @param value - width, height or corner radius
 * @returns `true` when the value is a safe integer ≥ 0
 * @see docs/domain/shapes.md#_3-creation-tools
 */
export function isModelSize(value: number): boolean {
  return Number.isSafeInteger(value) && value >= 0;
}
