/** A line's place in its block (`lineBaseline`): its index from 0, and the ordinate of the block's top. */
export type LineInBlock = {
  /** The line's place, from 0. */
  readonly index: number;
  /** The ordinate of its block's top, in the SVG. */
  readonly top: number;
};
