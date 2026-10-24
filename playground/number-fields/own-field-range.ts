/** The values a field acting as a number field of its own takes (US-087, AUD-046). */
export type OwnFieldRange = {
  /** Whether a value beyond the range goes round it, an angle's turn, rather than to its bound. */
  readonly isTurn: boolean;
  /** Whether it holds whole numbers, its value always rounded to the nearest integer (AUD-047). */
  readonly isWhole: boolean;
  /** Its least value, `-Infinity` for none. */
  readonly least: number;
  /** Its most value, `Infinity` for none; for a turn, the value that goes round to the least. */
  readonly most: number;
};
