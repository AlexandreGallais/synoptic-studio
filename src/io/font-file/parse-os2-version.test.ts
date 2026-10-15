import { describe, expect, it } from "vitest";

import { os2Table, viewOf } from "../../../tests";

import { parseOs2Version } from "./parse-os2-version";

describe("parseOs2Version (REF-OT-OS2)", () => {
  it.each([0, 1, 3, 5])("reads version %i from the table's first two bytes", (version) => {
    const os2 = os2Table({
      capHeight: 0,
      fsType: 0,
      strikeoutPosition: 0,
      strikeoutSize: 0,
      version,
    });

    expect(parseOs2Version(viewOf(os2))).toBe(version);
  });

  it("reads the two bytes big-endian: 0x0102 is 258", () => {
    expect(parseOs2Version(viewOf([0x01, 0x02]))).toBe(258);
  });
});
