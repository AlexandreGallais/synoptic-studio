import { characterUnits } from "./character-units";

/**
 * The number of typographic character units of a line, between which letter spacing is added
 * (`characterUnits`, `DERIV-text-width`, step 3).
 *
 * @kind domain
 * @param line - the characters measured of a line
 * @returns the number of its units; 0 for an empty line
 * @see REF-CSS-TEXT-3
 */
export function characterUnitCount(line: string): number {
  return characterUnits(line).length;
}
