import { describe, expect, it } from "vitest";

import { valueFormatToSize } from "./value-format-to-size";

describe("valueFormatToSize (REF-OT-GPOS)", () => {
  it.each([
    { expected: 0, valueFormat: 0x00_00 },
    // X_ADVANCE alone: one field.
    { expected: 2, valueFormat: 0x00_04 },
    // X_PLACEMENT and X_ADVANCE: two fields.
    { expected: 4, valueFormat: 0x00_05 },
    // The eight flags: eight fields.
    { expected: 16, valueFormat: 0x00_ff },
    // Reserved bits 8 to 15 add no field.
    { expected: 2, valueFormat: 0x01_04 },
  ])("gives $expected bytes for the ValueFormat $valueFormat", ({ expected, valueFormat }) => {
    expect(valueFormatToSize(valueFormat)).toBe(expected);
  });
});
