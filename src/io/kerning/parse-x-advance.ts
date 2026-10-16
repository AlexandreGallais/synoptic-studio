import { elementAddress, isBitSet, modulo, populationCount } from "../../math";

/**
 * The flags of the fields before `xAdvance`, X_PLACEMENT (bit 0) and Y_PLACEMENT (bit 1): the
 * remainder by 2² keeps them alone.
 */
const FIELDS_BEFORE_X_ADVANCE = 0b100;

/** Flag X_ADVANCE, bit 2. */
const X_ADVANCE_BIT = 2;

/** Size of each field of a ValueRecord, in bytes. */
const FIELD_SIZE = 2;

/**
 * Reads the horizontal advance adjustment of a ValueRecord: its `xAdvance`, after the placement
 * fields its ValueFormat holds; 0 when it holds none.
 *
 * @kind format
 * @param view - the `GPOS` table
 * @param record - where the ValueRecord starts in the table
 * @param valueFormat - its ValueFormat flags
 * @returns the adjustment, font units
 * @see REF-OT-GPOS
 */
export function parseXAdvance(view: DataView, record: number, valueFormat: number): number {
  const before = populationCount(modulo(valueFormat, FIELDS_BEFORE_X_ADVANCE));

  return isBitSet(valueFormat, X_ADVANCE_BIT)
    ? view.getInt16(elementAddress(record, FIELD_SIZE, before))
    : 0;
}
