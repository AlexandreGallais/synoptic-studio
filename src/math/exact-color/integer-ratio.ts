/** A fraction of two integers, kept apart so that its value is exact. */
export type IntegerRatio = {
  /** The numerator, an integer. */
  readonly numerator: number;
  /** The denominator, an integer > 0. */
  readonly denominator: number;
};
