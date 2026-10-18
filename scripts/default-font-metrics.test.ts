import { readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

import {
  DEFAULT_FONT_DATA,
  INTER_WEIGHTS,
  familyJson,
  interFaces,
  interFileName,
} from "./default-font-metrics.ts";

describe("default font metrics (EN-031)", () => {
  it("names the files of Inter 4.1's faces", () => {
    expect(interFileName("Bold", false)).toBe("Inter-Bold.ttf");
    expect(interFileName("Bold", true)).toBe("Inter-BoldItalic.ttf");
    expect(interFileName("Regular", false)).toBe("Inter-Regular.ttf");
    expect(interFileName("Regular", true)).toBe("Inter-Italic.ttf");
  });

  it("lists the nine weights, upright then italic", () => {
    const faces = interFaces();

    expect(faces).toHaveLength(2 * INTER_WEIGHTS.length);
    expect(faces[0]).toEqual({ file: "Inter-Thin.ttf", isItalic: false, weight: 100 });
    expect(faces.at(-1)).toEqual({ file: "Inter-BlackItalic.ttf", isItalic: true, weight: 900 });
  });

  it("writes the family as one line of JSON", () => {
    expect(familyJson({ faces: [], name: "Inter", version: "4.1" })).toBe(
      '{"faces":[],"name":"Inter","version":"4.1"}\n',
    );
  });

  it("keeps Inter's copyright notice and the OFL beside the data file", () => {
    const licence = readFileSync(join(DEFAULT_FONT_DATA, "..", "inter-license.txt"), "utf8");

    expect(licence.startsWith("Copyright (c) 2016 The Inter Project Authors")).toBe(true);
    expect(licence).toContain("SIL OPEN FONT LICENSE Version 1.1");
  });
});
