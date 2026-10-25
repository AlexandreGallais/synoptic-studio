/** Offset of `unitsPerEm` in the `head` table. */
const UNITS_PER_EM_OFFSET = 18;

/**
 * Reads the units per em of a font from its `head` table.
 *
 * @kind format
 * @param head - the font's `head` table, at least 20 bytes long
 * @returns the units per em, from 16 to 16 384 in a valid font
 */
export function parseUnitsPerEm(head: Readonly<DataView>): number {
  return head.getUint16(UNITS_PER_EM_OFFSET);
}
