/**
 * The values an open picker holds: its hue, kept while it is open even for a grey, and the
 * saturation and lightness or brightness of its plane (ADR-0056, decision 3).
 */
export type PickedColor = {
  /** The hue, in degrees, from 0 to 359.9. */
  readonly hue: number;
  /** The saturation, in percent, from 0 to 100. */
  readonly saturation: number;
  /** The lightness in HSL or the brightness in HSB, in percent, from 0 to 100. */
  readonly level: number;
};
