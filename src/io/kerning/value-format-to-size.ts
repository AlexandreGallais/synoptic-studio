import { isBitSet } from "../../math";

/** Number of defined flags of a ValueFormat, bits 0 to 7, one field of the ValueRecord each. */
const VALUE_FORMAT_FLAGS = 8;

/** Size of each field of a ValueRecord, in bytes. */
const FIELD_SIZE = 2;

/**
 * The size of the ValueRecords a ValueFormat describes: each flag set adds a field of two bytes.
 *
 * @kind format
 * @param valueFormat - the ValueFormat flags
 * @returns the size of one ValueRecord, in bytes
 * @see REF-OT-GPOS
 */
export function valueFormatToSize(valueFormat: number): number {
  return (
    FIELD_SIZE *
    Array.from({ length: VALUE_FORMAT_FLAGS }, (unused, bit) => bit).filter((bit) =>
      isBitSet(valueFormat, bit),
    ).length
  );
}
