import { caseFoldedFamilyName } from "./case-folded-family-name";

/**
 * Whether two font family names name the same family: CSS matches family names case
 * insensitively, without normalizing them (CSS Fonts 4, §5.1), so "arial" names Arial.
 *
 * @kind domain
 * @param name - a family name, as a text names it
 * @param other - a family name, as a font file names it
 * @returns `true` when they match without regard to case
 * @see REF-CSS-FONTS-4
 */
export function isSameFamilyName(name: string, other: string): boolean {
  return caseFoldedFamilyName(name) === caseFoldedFamilyName(other);
}
