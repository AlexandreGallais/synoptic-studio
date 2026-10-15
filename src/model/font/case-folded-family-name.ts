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
export function caseFoldedFamilyName(name: string): string {
  return name.toUpperCase().toLowerCase();
}
