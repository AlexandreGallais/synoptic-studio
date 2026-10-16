import { modulo, populationCount } from "../../math";

/** The defined flags of a ValueFormat sit in bits 0 to 7: the remainder by 2⁸ keeps them alone. */
const DEFINED_FLAGS = 0x1_00;

/** Size of each field of a ValueRecord, in bytes. */
const FIELD_SIZE = 2;

/**
 * The size of the ValueRecords a ValueFormat describes: each of its eight defined flags that is set
 * adds a field of two bytes; the reserved bits 8 to 15 add none.
 *
 * @kind format
 * @param valueFormat - the ValueFormat flags
 * @returns the size of one ValueRecord, in bytes
 * @see REF-OT-GPOS
 */
export function valueFormatToSize(valueFormat: number): number {
  return FIELD_SIZE * populationCount(modulo(valueFormat, DEFINED_FLAGS));
}
