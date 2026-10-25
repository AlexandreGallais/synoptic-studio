import { integerQuotient } from "../../math";

/**
 * The bytes a move of one shape adds to the history: A, a rectangle 100 × 50 kept by the
 * page, 614 characters, 1 228 bytes as it was, its reference, 8, and the index of its selection,
 * 8 (`historySize`); the 120 × 80 the page opens with, 616 characters, adds 1 248.
 */
const MOVE_BYTES = 1244;

/** The moves held are written rounded down to tens, an estimate. */
const ROUNDING = 10;

/**
 * About how many moves of one shape a memory size holds: the size over the 1 244 bytes a
 * move of one shape adds (`MOVE_BYTES`), rounded down to tens, as the toolbar shows it beside the
 * size set: 100 000 000 bytes hold 80 385.9 moves, about 80 380 (SP-029's answer 6).
 *
 * @kind domain
 * @param size - the memory size set, in bytes, a whole number, 0 or more
 * @returns the moves held, a multiple of 10
 * @see RULE-interaction-7-navigation-and-history
 */
export function movesWithinSize(size: number): number {
  return integerQuotient(size, MOVE_BYTES * ROUNDING) * ROUNDING;
}
