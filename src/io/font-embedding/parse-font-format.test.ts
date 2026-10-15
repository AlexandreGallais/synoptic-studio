import { describe, expect, it } from "vitest";

import { parseFontFormat } from "./parse-font-format";

describe("parseFontFormat (REF-CSS-FONTS-4 §11.2, REF-OT-OTFF)", () => {
  it("[F15.AC2] names a file with TrueType outlines font/ttf and truetype", () => {
    // sfntVersion 0x00010000.
    expect(parseFontFormat(Uint8Array.from([0, 1, 0, 0, 0, 9]))).toEqual({
      hint: "truetype",
      mediaType: "font/ttf",
    });
  });

  it("[F15.AC2] names a file with CFF outlines, OTTO, font/otf and opentype", () => {
    // "OTTO" = 0x4F 0x54 0x54 0x4F.
    expect(parseFontFormat(Uint8Array.from([0x4f, 0x54, 0x54, 0x4f, 0, 9]))).toEqual({
      hint: "opentype",
      mediaType: "font/otf",
    });
  });

  it("reads the first bytes of a view, and takes a file too short as TrueType", () => {
    // "xOTTO" viewed from 1 is "OTTO"; three bytes hold no sfntVersion.
    const bytes = Uint8Array.from([0x78, 0x4f, 0x54, 0x54, 0x4f]);

    expect(parseFontFormat(bytes.subarray(1)).hint).toBe("opentype");
    expect(parseFontFormat(Uint8Array.from([0x4f, 0x54, 0x54])).hint).toBe("truetype");
  });
});
