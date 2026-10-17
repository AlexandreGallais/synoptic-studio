import { GENERIC_FONT_FAMILIES } from "./generic-font-families";

/**
 * Whether a family name is one of CSS's generic families, in any case and with white space at
 * its ends (CSS Fonts 4, §2.1.2): `font-family` writes it as a keyword, and an `@font-face` rule
 * may not define it, so that a family named `Monospace` is never embedded.
 *
 * @kind domain
 * @param name - a family name
 * @returns true for `serif`, `sans-serif`, `monospace` and the other generic keywords
 * @see REF-CSS-FONTS-4
 */
export function isGenericFamilyName(name: string): boolean {
  return GENERIC_FONT_FAMILIES.includes(name.trim().toLowerCase());
}
