/**
 * How many states the list of the history's steps shows on each side of the present.
 */
export type HistoryWindow = {
  /** The states to come shown after the present. */
  readonly after: number;
  /** The past states shown before the present. */
  readonly before: number;
};
