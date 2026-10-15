/**
 * A color in HSB or HSL given to one decimal, each value counted in tenths, integers: 30.1° is
 * 301, 66.7 % is 667 (`DERIV-color-spaces`, step 11).
 */
export type TenthsColor = {
  /** The hue, in tenths of a degree, from 0 to 3599. */
  readonly hue: number;
  /** The saturation, in tenths of a percent, from 0 to 1000. */
  readonly saturation: number;
  /** The brightness in HSB or the lightness in HSL, in tenths of a percent, from 0 to 1000. */
  readonly level: number;
};
