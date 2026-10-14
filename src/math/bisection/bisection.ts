import { bisectionBracket } from "./bisection-bracket";
import { BISECTION_STEPS } from "./bisection-steps";

/**
 * Bisection: the value of `[low, high]` where a predicate stops being true, the bracket halved
 * `BISECTION_STEPS` times, so that the error is at most `2⁻⁶⁰ (high − low)`, or one unit in the
 * last place of the result once a double can no longer be halved (about 54 steps for a bracket of 3). The predicate must be
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
  const [start, end] = bisectionBracket(isBelow, [low, high], BISECTION_STEPS);

  return (start + end) / 2;
}
