import { average } from "../arithmetic";

/**
 * The last bracket of a bisection: a bracket halved a number of times, always keeping the half
 * where the predicate turns from true to false; its width shrinks by 2 at each halving
 * (`REF-MATHWORLD-BISECTION`). Its lower end is a value where the predicate was seen true, or the
 * bracket's own lower end.
 *
 * @kind math
 * @param isBelow - whether a value lies below the one sought
 * @param bracket - lower end, where the predicate holds, then upper end
 * @param steps - how many times it is halved
 * @returns the last bracket, lower end then upper end
 * @see REF-MATHWORLD-BISECTION
 */
export function bisectionBracket(
  isBelow: (value: number) => boolean,
  bracket: readonly [number, number],
  steps: number,
): readonly [number, number] {
  return Array.from({ length: steps }).reduce<readonly [number, number]>(([from, to]) => {
    const middle = average(from, to);

    return isBelow(middle) ? [middle, to] : [from, middle];
  }, bracket);
}
