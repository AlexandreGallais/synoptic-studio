import { isBitSet } from "../../math";

/** Flags of the fields before `xAdvance`: X_PLACEMENT (bit 0), Y_PLACEMENT (bit 1). */
const FIELDS_BEFORE_X_ADVANCE = [0, 1];

/** Flag X_ADVANCE, bit 2. */
const X_ADVANCE_BIT = 2;

/** Size of each field of a ValueRecord, in bytes. */
const FIELD_SIZE = 2;

/**
 * Reads the horizontal advance adjustment of a ValueRecord: its `xAdvance`, after the placement
 * fields its ValueFormat holds; 0 when it holds none.
 *
 * @kind format
 * @param view - the font file
 * @param record - where the ValueRecord starts in the file
 * @param valueFormat - its ValueFormat flags
 * @returns the adjustment, font units
 * @see REF-OT-GPOS
 */
export function parseXAdvance(view: DataView, record: number, valueFormat: number): number {
  const before = FIELDS_BEFORE_X_ADVANCE.filter((bit) => isBitSet(valueFormat, bit)).length;

  return isBitSet(valueFormat, X_ADVANCE_BIT) ? view.getInt16(record + FIELD_SIZE * before) : 0;
}
