/** A line of a paragraph that fits its box: the token it starts at and the room it leaves. */
export type FittingLine = {
  /** The index of its first token. */
  readonly start: number;
  /** The box's width less the line's, its trailing spaces left out, in user units. */
  readonly room: number;
};
