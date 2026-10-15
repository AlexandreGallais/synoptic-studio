import { describe, expect, it } from "vitest";

import { TRUE_TYPE_VERSION, fontFile, uint16, uint32, viewOf } from "../../../tests";

import { parseTableDirectory } from "./parse-table-directory";

describe("parseTableDirectory (REF-OT-OTFF)", () => {
  it("reads each table's offset and length by tag", () => {
    const file = fontFile(TRUE_TYPE_VERSION, { abcd: [1, 2, 3], wxyz: [4, 5, 6, 7, 8] });

    // Two records: 12 + 2 × 16 = 44 bytes; abcd (3 bytes) is padded to 4, so wxyz starts at 48.
    expect(parseTableDirectory(viewOf([...file]))).toEqual({
      abcd: { length: 3, offset: 44 },
      wxyz: { length: 5, offset: 48 },
    });
  });

  it("keeps a table that ends at the file's last byte", () => {
    // One record: 12 + 16 = 28 bytes; abcd from 28 to 32, the file's length.
    const file = fontFile(TRUE_TYPE_VERSION, { abcd: [1, 2, 3, 4] });

    expect(parseTableDirectory(viewOf([...file]))).toEqual({ abcd: { length: 4, offset: 28 } });
  });

  it("leaves out a table that ends past the file", () => {
    const file = [...fontFile(TRUE_TYPE_VERSION, { abcd: [1, 2, 3, 4] })];

    // The record's length, at 12 + 12 = 24, is set to 5: the table would end at 33 > 32.
    file.splice(24, 4, ...uint32(5));

    expect(parseTableDirectory(viewOf(file))).toEqual({});
  });

  it("reads nothing when the directory does not fit in the file", () => {
    // numTables 3 asks for 12 + 3 × 16 = 60 bytes; the file has 12.
    const file = [
      ...uint32(TRUE_TYPE_VERSION),
      ...uint16(3),
      ...uint16(0),
      ...uint16(0),
      ...uint16(0),
    ];

    expect(parseTableDirectory(viewOf(file))).toEqual({});
  });
});
