import { BISECTION_STEPS } from "./bisection-steps";

/**
 * Bisection: the value of `[low, high]` where a predicate stops being true, the bracket halved
 * `BISECTION_STEPS` times, so that the error is at most `2⁻⁶⁰ (high − low)`. The predicate must be
 * true below that value and false above it; true everywhere gives `high`, false everywhere `low`,
 * within the same error.
 *
 * @kind math
 * @param isBelow - whether a value lies below the one sought
 * @param low - lower end of the bracket
 * @param high - upper end of the bracket
 * @returns the middle of the last bracket
 * @see REF-MATHWORLD-BISECTION
 */
export function bisection(isBelow: (value: number) => boolean, low: number, high: number): number {
  const [start, end] = Array.from({ length: BISECTION_STEPS }).reduce<readonly [number, number]>(
    ([from, to]) => (isBelow((from + to) / 2) ? [(from + to) / 2, to] : [from, (from + to) / 2]),
    [low, high],
  );

  return (start + end) / 2;
}
