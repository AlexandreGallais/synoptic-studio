/** A text's lines shown and where they are laid out (`textFrame`): the top of their block, and the width they are aligned across. */
export type ShownBlock = {
  /** Its lines shown, in order (`shownLines`). */
  readonly lines: readonly string[];
  /** The ordinate of its block's top: the text's position, or lower in its box by its vertical alignment (`blockOffsetInBox`). */
  readonly top: number;
  /** The width its lines are aligned across: its widest line, or its box's width. */
  readonly width: number;
};
