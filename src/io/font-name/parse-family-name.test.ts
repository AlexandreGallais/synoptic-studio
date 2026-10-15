import { describe, expect, it } from "vitest";

import { ENGLISH_US, WINDOWS_PLATFORM, familyNameTable, nameTable, viewOf } from "../../../tests";

import { parseFamilyName } from "./parse-family-name";

/** A Windows name in English (United States). */
const ENGLISH_NAME = { language: ENGLISH_US, platform: WINDOWS_PLATFORM };

describe("parseFamilyName (REF-OT-NAME)", () => {
  it("[F15.AC1] names Arial by its name ID 1 when it has no name ID 16", () => {
    const name = viewOf(familyNameTable("Arial"));

    expect(parseFamilyName(name)).toBe("Arial");
  });

  it("[F15.AC1] names Inter by its name ID 16, not its name ID 1 Inter Light", () => {
    const name = nameTable([
      { ...ENGLISH_NAME, nameId: 1, text: "Inter Light" },
      { ...ENGLISH_NAME, nameId: 16, text: "Inter" },
    ]);

    expect(parseFamilyName(viewOf(name))).toBe("Inter");
  });

  it("gives undefined for a table naming no family", () => {
    const name = nameTable([{ ...ENGLISH_NAME, nameId: 2, text: "Regular" }]);

    expect(parseFamilyName(viewOf(name))).toBeUndefined();
  });
});
