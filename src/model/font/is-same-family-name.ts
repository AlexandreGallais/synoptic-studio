/**
 * A name folded for a caseless comparison: in upper case, then in lower case, which maps
 * "ß" to "ss" as full case folding does. An approximation of Unicode's Default Caseless
 * Matching, without normalization, as CSS asks (a few characters fold differently, such as the
 * Turkish dotless ı).
 *
 * @kind domain
 * @param name - a family name
 * @returns it, folded
 * @see REF-CSS-FONTS-4
 */
function caseFolded(name: string): string {
  return name.toUpperCase().toLowerCase();
}

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
  return caseFolded(name) === caseFolded(other);
}
