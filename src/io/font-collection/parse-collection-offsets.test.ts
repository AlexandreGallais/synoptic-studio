import { describe, expect, it } from "vitest";

import {
  SHARED_SANS_TABLES,
  TRUE_TYPE_VERSION,
  fontCollection,
  fontFile,
  sharedTableCollection,
  tagBytes,
  uint16,
  uint32,
  viewOf,
} from "../../../tests";

import { parseCollectionOffsets } from "./parse-collection-offsets";

describe("parseCollectionOffsets (REF-OT-OTFF)", () => {
  it("[F17.AC4] reads where each face of a collection sits", () => {
    // Header: tag, versions, numFonts (12 bytes), then 2 offsets (8 bytes) = 20; the first face,
    // 28 + 4 = 32 bytes, at 20; the second at 20 + 32 = 52.
    const collection = fontCollection([
      fontFile(TRUE_TYPE_VERSION, { abcd: [1, 2, 3] }),
      fontFile(TRUE_TYPE_VERSION, { wxyz: [4, 5, 6, 7, 8] }),
    ]);

    expect(parseCollectionOffsets(viewOf([...collection]))).toEqual([20, 52]);
  });

  it("[F17.AC4] [F18.AC19] reads the 3 faces of a collection sharing Inter's tables, at 24, 308 and 592", () => {
    // Header 12 + 3 × 4 = 24 bytes; each directory of Inter's 17 tables 12 + 17 × 16 = 284 bytes,
    // all three before the tables: at 24, 24 + 284 = 308 and 308 + 284 = 592.
    const file = sharedTableCollection(TRUE_TYPE_VERSION, SHARED_SANS_TABLES);

    expect(parseCollectionOffsets(new DataView(file.buffer))).toEqual([24, 308, 592]);
  });

  it("reads no face in a font file", () => {
    const file = [...fontFile(TRUE_TYPE_VERSION, {})];

    expect(parseCollectionOffsets(viewOf(file))).toEqual([]);
  });

  it("reads the offsets that end with the file", () => {
    // numFonts 1 asks for 12 + 1 × 4 = 16 bytes; the file has 16.
    const file = [...tagBytes("ttcf"), ...uint16(1), ...uint16(0), ...uint32(1), ...uint32(16)];

    expect(parseCollectionOffsets(viewOf(file))).toEqual([16]);
  });

  it("reads no face when the offsets do not fit in the file", () => {
    // numFonts 2 asks for 12 + 2 × 4 = 20 bytes; the file has 16.
    const file = [...tagBytes("ttcf"), ...uint16(1), ...uint16(0), ...uint32(2), ...uint32(20)];

    expect(parseCollectionOffsets(viewOf(file))).toEqual([]);
  });

  it("reads no face in a file shorter than a collection's header", () => {
    const file = [...tagBytes("ttcf"), ...uint16(1)];

    expect(parseCollectionOffsets(viewOf(file))).toEqual([]);
  });
});
