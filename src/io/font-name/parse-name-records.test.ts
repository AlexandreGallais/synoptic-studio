import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import {
  ENGLISH_US,
  WINDOWS_PLATFORM,
  nameTable,
  uint16,
  viewOf,
  viewOfFirst,
} from "../../../tests";

import { parseNameRecords } from "./parse-name-records";

/** A Windows name in English (United States), name ID 1. */
const ARIAL = {
  encoding: 1,
  language: ENGLISH_US,
  nameId: 1,
  platform: WINDOWS_PLATFORM,
  text: "Arial",
};

/** A name record drawn at random: its first four fields, then its string's length and offset. */
const RANDOM_RECORD = fc.record({
  fields: fc.array(fc.integer({ max: 0xff_ff, min: 0 }), { maxLength: 4, minLength: 4 }),
  length: fc.integer({ max: 80, min: 0 }),
  offset: fc.integer({ max: 80, min: 0 }),
});

describe("parseNameRecords (REF-OT-NAME)", () => {
  it("reads each record, its string placed from the table's start", () => {
    const bytes = nameTable([ARIAL, { ...ARIAL, nameId: 2, text: "Bold" }]);

    // Header 6 + two records of 12 = storage at 30; "Arial" 10 bytes at 30, "Bold" 8 at 40.
    expect(parseNameRecords(viewOf(bytes))).toEqual([
      { encoding: 1, language: ENGLISH_US, length: 10, nameId: 1, offset: 30, platform: 3 },
      { encoding: 1, language: ENGLISH_US, length: 8, nameId: 2, offset: 40, platform: 3 },
    ]);
  });

  it("gives no record, never an error, for a table too short for its header or records", () => {
    const bytes = nameTable([ARIAL]);

    expect(parseNameRecords(viewOf([]))).toEqual([]);
    expect(parseNameRecords(viewOf([0, 0, 0, 1, 0]))).toEqual([]);
    // Header 6 + one record 12 = 18 bytes: 17 cut the record.
    expect(parseNameRecords(viewOfFirst(bytes, 17))).toEqual([]);
  });

  it("leaves out a record whose string passes the table's end", () => {
    const bytes = nameTable([ARIAL]);

    // The string's 10 bytes end the table: one byte fewer cuts it.
    expect(parseNameRecords(viewOfFirst(bytes, -1))).toEqual([]);
    expect(parseNameRecords(viewOf(bytes))).toHaveLength(1);
  });

  test.prop({ bytes: fc.array(fc.integer({ max: 255, min: 0 }), { maxLength: 80 }) })(
    "reads any bytes without an error",
    ({ bytes }) => {
      expect(() => parseNameRecords(viewOf(bytes))).not.toThrow();
    },
  );

  test.prop({
    records: fc.array(RANDOM_RECORD, { maxLength: 8 }),
    storage: fc.array(fc.integer({ max: 255, min: 0 }), { maxLength: 64 }),
    storageOffset: fc.integer({ max: 120, min: 0 }),
  })(
    "keeps from a valid header and random records exactly those whose string lies in the table",
    ({ records, storage, storageOffset }) => {
      // Each record: platform, encoding, language and name ID at random, then length and offset.
      const recordBytes = records.flatMap((record) =>
        [...record.fields, record.length, record.offset].flatMap((field) => uint16(field)),
      );
      const bytes = [
        ...uint16(0),
        ...uint16(records.length),
        ...uint16(storageOffset),
        ...recordBytes,
        ...storage,
      ];
      const inside = records.filter(
        (record) => storageOffset + record.offset + record.length <= bytes.length,
      );

      expect(parseNameRecords(viewOf(bytes)).map((record) => record.length)).toEqual(
        inside.map((record) => record.length),
      );
    },
  );
});
