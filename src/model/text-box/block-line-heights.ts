/** The heights a block of lines is measured with (`DERIV-text-box`, step 5), in user units. */
export type BlockLineHeights = {
  /** The height of its first line: the line height, or the cap height when the block is trimmed. */
  readonly firstLine: number;
  /** The line height, from one baseline to the next. */
  readonly line: number;
};
