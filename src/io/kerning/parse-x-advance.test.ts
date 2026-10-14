import { describe, expect, it } from "vitest";

import { int16, viewOf } from "../../../tests";

import { parseXAdvance } from "./parse-x-advance";

/** A ValueRecord's fields in order: 11, 22, −33. */
const RECORD = viewOf([...int16(11), ...int16(22), ...int16(-33)]);

describe("parseXAdvance (REF-OT-GPOS)", () => {
  it.each([
    // X_ADVANCE alone: the first field.
    { expected: 11, valueFormat: 0x00_04 },
    // X_PLACEMENT, X_ADVANCE: the second field.
    { expected: 22, valueFormat: 0x00_05 },
    // Y_PLACEMENT, X_ADVANCE: the second field.
    { expected: 22, valueFormat: 0x00_06 },
    // X_PLACEMENT, Y_PLACEMENT, X_ADVANCE: the third field.
    { expected: -33, valueFormat: 0x00_07 },
    // No X_ADVANCE: no adjustment.
    { expected: 0, valueFormat: 0x00_01 },
  ])("reads $expected for the ValueFormat $valueFormat", ({ expected, valueFormat }) => {
    expect(parseXAdvance(RECORD, 0, valueFormat)).toBe(expected);
  });
});
