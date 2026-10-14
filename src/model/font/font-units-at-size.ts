/**
 * A length in font units, in user units at a text's size: the em is the size and holds
 * `unitsPerEm` font units, so one font unit is `size / unitsPerEm` (`DERIV-text-width`, step 1).
 *
 * Formula: units × size / unitsPerEm, multiplied first so that a whole result stays whole.
 *
 * @kind domain
 * @param units - the length in font units
 * @param size - the text's size, the em in user units
 * @param unitsPerEm - the font's units per em, from 16 to 16 384
 * @returns the length in user units
 * @see REF-OT-HEAD
 */
export function fontUnitsAtSize(units: number, size: number, unitsPerEm: number): number {
  return (units * size) / unitsPerEm;
}
