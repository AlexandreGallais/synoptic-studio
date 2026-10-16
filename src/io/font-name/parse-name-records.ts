import { elementAddress, isSpanWithin } from "../../math";

import type { NameRecord } from "./name-record";

/** Size of the `name` table's header: `version`, `count` and `storageOffset`. */
const NAME_HEADER_SIZE = 6;

/** Offset of `count` in the `name` table. */
const COUNT_OFFSET = 2;

/** Offset of `storageOffset` in the `name` table. */
const STORAGE_OFFSET = 4;

/** Size of one name record: six fields of two bytes. */
const NAME_RECORD_SIZE = 12;

/** Offsets of a name record's fields, from the record's start. */
const RECORD_FIELDS = { encoding: 2, language: 4, length: 8, nameId: 6, offset: 10, platform: 0 };

/**
 * How many name records the `name` table holds: none when its header, or its records, do not fit
 * in it.
 *
 * @kind format
 * @param name - the font's `name` table
 * @returns its number of records
 * @see REF-OT-NAME
 */
function parseRecordCount(name: DataView): number {
  const count = isSpanWithin(0, NAME_HEADER_SIZE, name.byteLength)
    ? name.getUint16(COUNT_OFFSET)
    : 0;

  return isSpanWithin(0, elementAddress(NAME_HEADER_SIZE, NAME_RECORD_SIZE, count), name.byteLength)
    ? count
    : 0;
}

/**
 * Reads one name record, its string's offset counted from the storage area's start
 * (`storageOffset`) and returned from the table's start.
 *
 * @kind format
 * @param name - the font's `name` table, holding the record
 * @param at - where the record starts, in bytes
 * @returns its platform, encoding, language and name ID, and where its string lies
 * @see REF-OT-NAME
 */
function parseNameRecord(name: DataView, at: number): NameRecord {
  return {
    encoding: name.getUint16(at + RECORD_FIELDS.encoding),
    language: name.getUint16(at + RECORD_FIELDS.language),
    length: name.getUint16(at + RECORD_FIELDS.length),
    nameId: name.getUint16(at + RECORD_FIELDS.nameId),
    offset: name.getUint16(STORAGE_OFFSET) + name.getUint16(at + RECORD_FIELDS.offset),
    platform: name.getUint16(at + RECORD_FIELDS.platform),
  };
}

/**
 * Reads the name records of a font's `name` table, in the table's order; a record whose string
 * passes the table's end is left out, and a table too short for its header or its records has
 * none. Every read is checked against the table: never an error.
 *
 * @kind format
 * @param name - the font's `name` table
 * @returns its records whose string lies in the table
 * @see REF-OT-NAME
 */
export function parseNameRecords(name: DataView): readonly NameRecord[] {
  return Array.from({ length: parseRecordCount(name) }, (unused, index) =>
    parseNameRecord(name, elementAddress(NAME_HEADER_SIZE, NAME_RECORD_SIZE, index)),
  ).filter((record) => isSpanWithin(record.offset, record.length, name.byteLength));
}
