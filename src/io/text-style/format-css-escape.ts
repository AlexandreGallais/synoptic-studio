/** The base the code of an escaped character is written in. */
const HEXADECIMAL = 16;

/** U+FFFD, the replacement character, written for a string that holds no character (CSS 2.1, §4.1.3). */
const REPLACEMENT_CHARACTER = 0xff_fd;

/**
 * A character escaped in a CSS string: a backslash, then its code in hexadecimal and one space,
 * which ends the code and is not part of the string (CSS 2.1, §4.1.3): a line feed is `\A `, a
 * tab `\9 `.
 *
 * @kind format
 * @param character - one character; an empty string gives the replacement character's escape
 * @returns its escape
 */
export function formatCssEscape(character: string): string {
  const code = character.codePointAt(0) ?? REPLACEMENT_CHARACTER;

  return `\\${code.toString(HEXADECIMAL).toUpperCase()} `;
}
