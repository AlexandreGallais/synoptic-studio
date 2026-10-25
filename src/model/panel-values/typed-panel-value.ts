/**
 * A value typed in a field acting as a number field: a number, relative to the
 * field's own value, or each shape's, when typed with a sign, `+50` or `-20`, otherwise the value
 * set; a fraction kept, for the field to round once its value is computed.
 */
export type TypedPanelValue = {
  /** Whether it was typed with a sign, and is then added to the own value. */
  readonly isRelative: boolean;
  /** The number typed, its sign included: `-20` for `-20`, `1.5` for `1.5`. */
  readonly value: number;
};
