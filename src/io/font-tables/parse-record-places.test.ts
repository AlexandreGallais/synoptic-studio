import { describe, expect, it } from "vitest";

import {
  APPLE_TRUE_TYPE_VERSION,
  OPEN_TYPE_VERSION,
  TRUE_TYPE_VERSION,
  fontCollection,
  fontFile,
  uint32,
  viewOf,
} from "../../../tests";

import { parseRecordPlaces } from "./parse-record-places";

/** Two tables: records at 12 and 12 + 16 = 28. */
const TABLES = { abcd: [1, 2, 3], wxyz: [4, 5, 6, 7, 8] };

describe("parseRecordPlaces (REF-OT-OTFF, REF-APPLE-TT-SCALER)", () => {
  it.each([
    { name: "0x00010000", version: TRUE_TYPE_VERSION },
    { name: "OTTO", version: OPEN_TYPE_VERSION },
    { name: "true", version: APPLE_TRUE_TYPE_VERSION },
  ])("[F17.AC4] reads the records of a font whose sfntVersion is $name", ({ version }) => {
    const file = [...fontFile(version, TABLES)];

    expect(parseRecordPlaces(viewOf(file), 0)).toEqual([12, 28]);
  });

  it("[F17.AC4] reads nothing for a sfntVersion of another kind, 'typ1'", () => {
    const file = [...fontFile(TRUE_TYPE_VERSION, TABLES)];

    // 'typ1' = 0x74797031, a PostScript font in an sfnt wrapper: not read.
    file.splice(0, 4, ...uint32(0x74_79_70_31));

    expect(parseRecordPlaces(viewOf(file), 0)).toEqual([]);
  });

  it("[F17.AC4] reads a face's records at its directory in a collection", () => {
    // Header 12 + 2 × 4 = 20 bytes; the first face (32 bytes) at 20, the second at 52; the
    // second's one record at 52 + 12 = 64.
    const collection = fontCollection([
      fontFile(TRUE_TYPE_VERSION, { abcd: [1, 2, 3] }),
      fontFile(TRUE_TYPE_VERSION, { wxyz: [4, 5, 6, 7, 8] }),
    ]);

    expect(parseRecordPlaces(viewOf([...collection]), 52)).toEqual([64]);
  });

  it("reads the records of a directory that ends with the file", () => {
    // Header 12 + 1 record × 16 = 28 bytes: the file cut at 28 still holds the directory.
    const file = [...fontFile(TRUE_TYPE_VERSION, { abcd: [1, 2, 3] })].slice(0, 28);

    expect(parseRecordPlaces(viewOf(file), 0)).toEqual([12]);
  });

  it("reads nothing at a directory whose header ends past the file", () => {
    // 32 bytes: a directory at 24 would end its header at 36.
    const file = [...fontFile(TRUE_TYPE_VERSION, { abcd: [1, 2, 3] })];

    expect(parseRecordPlaces(viewOf(file), 24)).toEqual([]);
    // At 20 the header fits (it ends at 32), but its first four bytes are abcd's offset, 28.
    expect(parseRecordPlaces(viewOf(file), 20)).toEqual([]);
  });
});
