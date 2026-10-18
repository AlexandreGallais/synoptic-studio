import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { os2FaceTable, viewOf, viewOfFirst } from "../../../tests";

import { parseFontStyle } from "./parse-font-style";

/** An `OS/2` table of version 3 holding Arial 5.01's weight and style: 400, upright. */
const ARIAL_OS2 = {
  capHeight: 1467,
  fsSelection: 0x00_40,
  fsType: 8,
  strikeoutPosition: 0,
  strikeoutSize: 0,
  version: 3,
  weightClass: 400,
};

describe("parseFontStyle (REF-OT-OS2)", () => {
  it("[F15.AC1] reads Arial's weight 400, upright, from fsSelection 0x0040", () => {
    const os2 = viewOf(os2FaceTable(ARIAL_OS2));

    expect(parseFontStyle(os2)).toEqual({
      isItalic: false,
      weight: 400,
    });
  });

  it.each([
    { weight: 300, weightClass: 300 },
    { weight: 100, weightClass: 50 },
    { weight: 900, weightClass: 950 },
  ])("[F15.AC1] reads usWeightClass $weightClass as the weight $weight", (row) => {
    const os2 = viewOf(os2FaceTable({ ...ARIAL_OS2, weightClass: row.weightClass }));

    expect(parseFontStyle(os2)?.weight).toBe(row.weight);
  });

  it.each([
    // Bit 0, ITALIC; bit 6, REGULAR; bit 5, BOLD, says nothing of the style.
    { fsSelection: 0x00_01, isItalic: true },
    { fsSelection: 0x00_40, isItalic: false },
    { fsSelection: 0x00_21, isItalic: true },
    { fsSelection: 0x00_20, isItalic: false },
    { fsSelection: 0x00_00, isItalic: false },
  ])("[F15.AC1] reads fsSelection $fsSelection as italic: $isItalic", (row) => {
    const os2 = viewOf(os2FaceTable({ ...ARIAL_OS2, fsSelection: row.fsSelection }));

    expect(parseFontStyle(os2)?.isItalic).toBe(row.isItalic);
  });

  it("reads a table of exactly 64 bytes, fsSelection its last two", () => {
    const os2 = viewOfFirst(os2FaceTable({ ...ARIAL_OS2, fsSelection: 1 }), 64);

    expect(parseFontStyle(os2)).toEqual({ isItalic: true, weight: 400 });
  });

  it("gives undefined, never an error, for a table too short to hold fsSelection", () => {
    // 63 bytes: fsSelection, at 62 and 63, is cut.
    const cut = viewOfFirst(os2FaceTable(ARIAL_OS2), 63);

    expect(parseFontStyle(cut)).toBeUndefined();
    expect(parseFontStyle(viewOf([]))).toBeUndefined();
  });

  test.prop({
    fsSelection: fc.integer({ max: 65_535, min: 0 }),
    weightClass: fc.integer({ max: 65_535, min: 0 }),
  })("is italic exactly when fsSelection is odd, its weight from 100 to 900", (fields) => {
    const os2 = viewOf(os2FaceTable({ ...ARIAL_OS2, ...fields }));
    const style = parseFontStyle(os2);

    expect(style?.isItalic).toBe(fields.fsSelection % 2 === 1);
    expect(style?.weight).toBeGreaterThanOrEqual(100);
    expect(style?.weight).toBeLessThanOrEqual(900);
  });
});
