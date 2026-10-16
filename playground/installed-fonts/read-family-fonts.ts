import { isSameFamilyName } from "../../src";

import type { FontSession, InstalledFont } from "../fonts";

/**
 * Reads the installed faces of a family whose files are not held: those read ahead, their metrics
 * kept without their files, under the name their files give (US-053); a family listed and not
 * read yet is never offered in the font list (US-059).
 *
 * @kind procedure
 * @param session - the families kept and the installed faces read ahead
 * @param family - a family of the font list
 * @returns its faces, in the browser's order; none for a family whose files are held or that is not installed
 * @see REF-WICG-LOCAL-FONT-ACCESS
 */
export function readFamilyFonts(session: FontSession, family: string): readonly InstalledFont[] {
  return session.readAhead
    .filter((read) => isSameFamilyName(read.family, family))
    .map((read) => read.font);
}
