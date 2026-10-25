import type { FamilyOutcome } from "./family-outcome";
import type { LeftOutFamily } from "./left-out-family";
import type { ReadAheadCount } from "./read-ahead-count";

/**
 * Reads the families left out with one family more, when reading it left it out.
 *
 * @kind procedure
 * @param leftOut - the families left out before it
 * @param family - its name, as the browser lists it
 * @param outcome - what reading it gave
 * @returns the families left out, it last with its reason when it is left out
 */
function readLeftOutWith(
  leftOut: readonly LeftOutFamily[],
  family: string,
  outcome: FamilyOutcome,
): readonly LeftOutFamily[] {
  return outcome === "read" ? leftOut : [...leftOut, { family, reason: outcome }];
}

/**
 * Counts one family more.
 *
 * @kind procedure
 * @param count - what was counted before it
 * @param family - its name, as the browser lists it
 * @param outcome - what reading it gave
 * @returns the count with it
 */
export function readCountWith(
  count: ReadAheadCount,
  family: string,
  outcome: FamilyOutcome,
): ReadAheadCount {
  return {
    ...count,
    done: count.done + 1,
    leftOut: readLeftOutWith(count.leftOut, family, outcome),
    read: outcome === "read" ? count.read + 1 : count.read,
  };
}
