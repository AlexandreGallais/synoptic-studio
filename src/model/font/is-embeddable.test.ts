import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isEmbeddable } from "./is-embeddable";

describe("isEmbeddable (DERIV-font-embedding, step 3)", () => {
  it.each([
    // Arial 5.01: bit 3, editable.
    { case: "8, Arial, version 3", fsType: 8, isAllowed: true, version: 3 },
    // Inter 4.1: no bit set, installable.
    { case: "0, Inter, version 4", fsType: 0, isAllowed: true, version: 4 },
    // Bit 2, preview and print.
    { case: "4", fsType: 4, isAllowed: true, version: 4 },
    // Bit 8, no subsetting: the whole file is not a subset.
    { case: "0x0100", fsType: 0x01_00, isAllowed: true, version: 4 },
    // Bit 0 is reserved, bits 1 to 3 clear: installable.
    { case: "1", fsType: 1, isAllowed: true, version: 4 },
    // 2 and 8: bit 3 wins, the least restrictive.
    { case: "10, version 2", fsType: 10, isAllowed: true, version: 2 },
    // Bit 1 alone: restricted licence.
    { case: "2", fsType: 2, isAllowed: false, version: 4 },
    // Version 4, bit 9: bitmap only, no outline embedded.
    { case: "0x0208, version 4", fsType: 0x02_08, isAllowed: false, version: 4 },
    // Version 1 ignores bits 4 to 15: bit 3, editable.
    { case: "0x0208, version 1", fsType: 0x02_08, isAllowed: true, version: 1 },
  ])("[F15.AC2] tells fsType $case embeddable: $isAllowed", ({ fsType, isAllowed, version }) => {
    expect(isEmbeddable(fsType, version)).toBe(isAllowed);
  });

  it("reads bit 9 from version 2, and ignores it in versions 0 and 1", () => {
    // 0x0200: bitmap only, nothing else set.
    expect([0, 1, 2, 3, 5].map((version) => isEmbeddable(0x02_00, version))).toEqual([
      true,
      true,
      false,
      false,
      false,
    ]);
  });

  test.prop({
    high: fc.integer({ max: 0x3f, min: 0 }),
    low: fc.integer({ max: 0x01_ff, min: 0 }),
    version: fc.integer({ max: 5, min: 2 }),
  })("[F15.AC2] always refuses a font whose bit 9 is set, from version 2", (fields) => {
    // Bits 10 to 15 from high, bit 9 set, bits 0 to 8 from low: any fsType holding bit 9.
    const fsType = fields.high * 0x04_00 + 0x02_00 + fields.low;

    expect(isEmbeddable(fsType, fields.version)).toBe(false);
  });

  test.prop({
    fsType: fc.integer({ max: 0x01_ff, min: 0 }),
    version: fc.integer({ max: 5, min: 0 }),
  })("without bit 9, refuses exactly bit 1 without bit 2 nor bit 3", ({ fsType, version }) => {
    // Bits 1, 2, 3: ⌊f / 2⌋ mod 8; only 1 (binary 001) is restricted.
    expect(isEmbeddable(fsType, version)).toBe(Math.floor(fsType / 2) % 8 !== 1);
  });
});
