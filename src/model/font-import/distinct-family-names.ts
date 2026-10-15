import { caseFoldedFamilyName } from "../font";

/**
 * The family names of a list, each family once: a name that matches one before it without regard
 * to case is left out, as CSS matches family names (`isSameFamilyName`, CSS Fonts 4, §5.1), so
 * that "arial" after Arial is not listed again. One pass, each name looked up by its case-folded
 * form among the first index of each: 590 installed faces in 186 families cost 590 lookups, not the
 * 590 × 589 / 2 comparisons of every name with each before it (US-048).
 *
 * @kind domain
 * @param names - family names, in order
 * @returns the first name of each family, in order
 * @see REF-CSS-FONTS-4
 */
export function distinctFamilyNames(names: readonly string[]): readonly string[] {
  const entries = names.map((name) => ({ key: caseFoldedFamilyName(name), name }));
  // Entries from the last name to the first: the first index of each folded name is set last.
  const firstIndexes = new Map(
    entries.map((entry, index): [string, number] => [entry.key, index]).toReversed(),
  );

  return entries
    .filter((entry, index) => firstIndexes.get(entry.key) === index)
    .map((entry) => entry.name);
}
