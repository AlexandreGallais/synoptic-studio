/**
 * A value typed in a field of several shapes (F38): an integer, relative to each shape's own when
 * typed with a sign, `+50` or `-20`, otherwise the value each shape is set to.
 */
export type TypedPanelValue = {
  /** Whether it was typed with a sign, and is then added to each shape's own value. */
  readonly isRelative: boolean;
  /** The integer typed, its sign included: `-20` for `-20`. */
  readonly value: number;
};
