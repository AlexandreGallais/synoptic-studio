import { describe, expect, it } from "vitest";

import { formatCssDeclarations } from "./format-css-declarations";

describe("formatCssDeclarations (REF-CSS-STYLE-ATTR, §3)", () => {
  it("joins each property and its value by a colon, the declarations by semicolons", () => {
    expect(
      formatCssDeclarations([
        ["font-size", "16px"],
        ["white-space", "pre"],
      ]),
    ).toBe("font-size: 16px; white-space: pre");
  });

  it("writes an empty value for no declaration", () => {
    expect(formatCssDeclarations([])).toBe("");
  });
});
