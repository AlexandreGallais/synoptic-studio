/**
 * The values an open picker holds: its hue, kept while it is open even for a grey, and the
 * saturation and lightness or brightness of its plane, whole numbers (ADR-0056, decision 3;
 * ADR-0058).
 */
export type PickedColor = {
  /** The hue, in whole degrees, from 0 to 359. */
  readonly hue: number;
  /** The saturation, in whole percent, from 0 to 100. */
  readonly saturation: number;
  /** The lightness in HSL or the brightness in HSB, in whole percent, from 0 to 100. */
  readonly level: number;
};
