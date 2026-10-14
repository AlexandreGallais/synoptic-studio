import { GENERIC_FONT_FAMILIES } from "./generic-font-families";

/** A backslash or a double quote, escaped inside a CSS string. */
const STRING_ESCAPED = /["\\]/gu;

/**
 * One name of a `font-family` list: a generic family as its keyword, any other family as a
 * quoted string, a backslash or a double quote in it escaped by a backslash, so that a name with
 * spaces, digits or punctuation, or one spelled like a keyword, is read as written (CSS Fonts 4,
 * §2.1.1 and §2.1.2; CSS Values 4, §4.4).
 *
 * @kind format
 * @param name - a family name, as the text names it
 * @returns `sans-serif` for that keyword in any case, else the name in double quotes
 * @see REF-CSS-FONTS-4
 */
export function formatFontFamilyName(name: string): string {
  const keyword = name.trim().toLowerCase();

  return GENERIC_FONT_FAMILIES.includes(keyword)
    ? keyword
    : `"${name.replaceAll(STRING_ESCAPED, (character) => `\\${character}`)}"`;
}
