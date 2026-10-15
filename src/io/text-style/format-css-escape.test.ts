import { describe, expect, it } from "vitest";

import { formatCssEscape } from "./format-css-escape";

describe("formatCssEscape (REF-CSS2-SYNDATA §4.1.3)", () => {
  it(String.raw`writes a line feed as \A and a tab as \9, each ended by a space`, () => {
    expect([formatCssEscape("\n"), formatCssEscape("\t")]).toEqual([
      String.raw`\A `,
      String.raw`\9 `,
    ]);
  });

  it(String.raw`writes DEL, U+007F, as \7F`, () => {
    expect(formatCssEscape("\u{7F}")).toBe(String.raw`\7F `);
  });

  it("writes the replacement character for a string that holds none", () => {
    expect(formatCssEscape("")).toBe(String.raw`\FFFD `);
  });
});
