/**
 * How far a shape or a text moves along one axis for its straight box to start at a value typed
 * in X or Y: the value minus where its straight box starts, its stored position, the top-left of
 * its box before it turns, going by that difference (ADR-0035).
 *
 * @kind domain
 * @param start - where its straight box starts on that axis
 * @param target - where it must start, the value typed
 * @returns the shift, in user units
 * @see RULE-shapes-rotation-of-shapes
 */
export function straightBoxShift(start: number, target: number): number {
  return target - start;
}
