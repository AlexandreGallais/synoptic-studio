/** A color's red, green and blue as fractions from 0 to 1, each byte divided by 255. */
export type Rgb = {
  /** The red, from 0 to 1. */
  readonly red: number;
  /** The green, from 0 to 1. */
  readonly green: number;
  /** The blue, from 0 to 1. */
  readonly blue: number;
};
