/** The first and last values a slider holds, which Home and End go to (`REF-WAI-APG-SLIDER`). */
export type SliderRange = {
  /** The first value, at the left end. */
  readonly minimum: number;
  /** The last value, at the right end. */
  readonly maximum: number;
};
