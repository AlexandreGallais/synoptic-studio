/** The values a field acting as a number field of its own takes. */
export type OwnFieldRange = {
  /** Whether a value beyond the range goes round it, an angle's turn, rather than to its bound. */
  readonly isTurn: boolean;
  /** Whether it holds whole numbers, its value always rounded to the nearest integer. */
  readonly isWhole: boolean;
  /** Its minimum, `-Infinity` for none. */
  readonly minimum: number;
  /** Its maximum, `Infinity` for none; for a turn, the value that goes round to the minimum. */
  readonly maximum: number;
};
