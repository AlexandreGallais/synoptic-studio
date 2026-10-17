import { isSpanWithin } from "../../math";

import { parseOs2Version } from "./parse-os2-version";

/** Offset of `sCapHeight` in the `OS/2` table, from version 2. */
const CAP_HEIGHT_OFFSET = 88;

/** The first version of the `OS/2` table that holds `sCapHeight`. */
const CAP_HEIGHT_VERSION = 2;

/** The cap height assumed when the font gives none, in em (`REF-CSS-INLINE-3`, Appendix A.2). */
const FALLBACK_CAP_HEIGHT_EM = 0.66;

/**
 * A length in em written in font units: the em is `unitsPerEm` units (`REF-OT-HEAD`).
 *
 * Formula: e × U.
 *
 * @kind format
 * @param em - the length, em
 * @param unitsPerEm - how many font units make the em
 * @returns the length, font units
 * @see REF-OT-HEAD
 */
function emToFontUnits(em: number, unitsPerEm: number): number {
  return em * unitsPerEm;
}

/**
 * Reads a font's cap height, "the distance between the baseline and the approximate height of
 * uppercase letters": `sCapHeight` of `OS/2`, defined from version 2 of the table and set to 0
 * when the font has no `H` (`REF-OT-OS2`). A table older than version 2, too short to hold the
 * field, or giving 0 or less, gives the fallback of CSS, 0.66 em (`REF-CSS-INLINE-3`,
 * Appendix A.2; `DERIV-text-box`, step 7).
 *
 * @kind format
 * @param os2 - the font's `OS/2` table, at least 30 bytes long
 * @param unitsPerEm - how many font units make the em
 * @returns the cap height, font units, above 0
 * @see REF-OT-OS2
 */
export function parseCapHeight(os2: Readonly<DataView>, unitsPerEm: number): number {
  const capHeight =
    isSpanWithin(CAP_HEIGHT_OFFSET, Int16Array.BYTES_PER_ELEMENT, os2.byteLength) &&
    parseOs2Version(os2) >= CAP_HEIGHT_VERSION
      ? os2.getInt16(CAP_HEIGHT_OFFSET)
      : 0;

  return capHeight > 0 ? capHeight : emToFontUnits(FALLBACK_CAP_HEIGHT_EM, unitsPerEm);
}
