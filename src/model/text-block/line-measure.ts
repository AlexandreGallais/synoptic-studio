/** What a line's width is made of, kept so that lines joined are measured without measuring them again (`DERIV-text-width`, steps 1 to 3). */
export type LineMeasure = {
  /** The line, as typed in the text's case. */
  readonly text: string;
  /** The sum of its characters' advances and kerning, in font units, the kerning before its first character included. */
  readonly advance: number;
  /** The number of its typographic character units, between which letter spacing is added. */
  readonly unitCount: number;
  /** The number of its word separators (space, no-break space), at which word spacing is added. */
  readonly separatorCount: number;
  /** The code point of its first character measured; −1 for none. */
  readonly first: number;
  /** The code point of its last character measured; −1 for none. */
  readonly last: number;
};
