import { firstOfEachClass } from "../../math";
import { caseFoldedFamilyName } from "../font";

/**
 * The family names of a list, each family once: a name that matches one before it without regard
 * to case is left out, as CSS matches family names (`isSameFamilyName`, CSS Fonts 4, §5.1), so
 * that "arial" after Arial is not listed again: the first of each class of names whose
 * case-folded forms are equal (`firstOfEachClass`), so that 590 installed faces in 186
 * families cost 590 lookups, not the 590 × 589 / 2 comparisons of every name with each before it
 * (US-048).
 *
 * @kind domain
 * @param names - family names, in order
 * @returns the first name of each family, in order
 * @see REF-CSS-FONTS-4
 */
export function distinctFamilyNames(names: readonly string[]): readonly string[] {
  return firstOfEachClass(names, caseFoldedFamilyName);
}
