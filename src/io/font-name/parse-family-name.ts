import { parseFontName } from "./parse-font-name";

/** Name ID 1, the Font Family name, shared by at most four fonts of a weight or a style. */
const FAMILY_NAME = 1;

/** Name ID 16, the typographic family name of an extended family. */
const TYPOGRAPHIC_FAMILY_NAME = 16;

/**
 * Reads the family of a font from its `name` table: the typographic family name (name ID 16),
 * which an extended family of more than four faces gives, else the family name (name ID 1) —
 * Inter Light's name ID 1 is `Inter Light`, its name ID 16 `Inter`.
 *
 * @kind format
 * @param name - the font's `name` table
 * @returns the family's name; undefined when the table names none
 * @see REF-OT-NAME
 */
export function parseFamilyName(name: Readonly<DataView>): string | undefined {
  return parseFontName(name, TYPOGRAPHIC_FAMILY_NAME) ?? parseFontName(name, FAMILY_NAME);
}
