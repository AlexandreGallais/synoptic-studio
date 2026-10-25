import { integerRange } from "../../math";

/** First letter of the ISO basic Latin alphabet's uppercase set, A, at hexadecimal 41. */
const FIRST_UPPERCASE = 0x41;

/** Last letter of its uppercase set, Z, at hexadecimal 5A. */
const LAST_UPPERCASE = 0x5a;

/** First letter of its lowercase set, a, at hexadecimal 61. */
const FIRST_LOWERCASE = 0x61;

/** Last letter of its lowercase set, z, at hexadecimal 7A. */
const LAST_LOWERCASE = 0x7a;

/**
 * The code points of the ISO basic Latin alphabet, its 26 uppercase letters then its 26
 * lowercase ones, at "positions in hexadecimal notation 41 to 5A for uppercase and 61 to 7A for
 * lowercase" in ISO/IEC 10646 (`REF-WIKI-ISO-BASIC-LATIN`): the letters a font must map for the
 * editor to list it, whatever its script (ADR-0061).
 */
export const BASIC_LATIN_LETTERS: readonly number[] = [
  ...integerRange(FIRST_UPPERCASE, LAST_UPPERCASE),
  ...integerRange(FIRST_LOWERCASE, LAST_LOWERCASE),
];
