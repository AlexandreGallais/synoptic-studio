/** The space, U+0020. */
const SPACE = 0x20;

/** The no-break space, U+00A0. */
const NO_BREAK_SPACE = 0xa0;

/** The word separators of the scripts measured. */
const WORD_SEPARATORS: ReadonlySet<number> = new Set([SPACE, NO_BREAK_SPACE]);

/**
 * Whether a character separates words, so that word spacing is added at it: the space (U+0020)
 * or the no-break space (U+00A0).
 *
 * @kind domain
 * @param codePoint - the character's code point
 * @returns `true` for a space or a no-break space
 * @see REF-CSS-TEXT-3
 */
export function isWordSeparator(codePoint: number): boolean {
  return WORD_SEPARATORS.has(codePoint);
}
