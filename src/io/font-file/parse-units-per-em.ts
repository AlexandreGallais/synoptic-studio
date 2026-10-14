import type { FontTableRecord } from "./font-table-record";

/** Offset of `unitsPerEm` in the `head` table. */
const UNITS_PER_EM_OFFSET = 18;

/**
 * Reads the units per em of a font from its `head` table.
 *
 * @kind format
 * @param view - the font file
 * @param head - where its `head` table sits
 * @returns the units per em, from 16 to 16 384 in a valid font
 * @see REF-OT-HEAD
 */
export function parseUnitsPerEm(view: DataView, head: FontTableRecord): number {
  return view.getUint16(head.offset + UNITS_PER_EM_OFFSET);
}
