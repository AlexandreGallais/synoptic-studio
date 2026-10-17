import { describe, expect, it } from "vitest";

import { isWordSeparator } from "./is-word-separator";

describe("isWordSeparator (REF-CSS-TEXT-3)", () => {
  it.each([
    { codePoint: 0x20, isSeparator: true, name: "the space" },
    { codePoint: 0xa0, isSeparator: true, name: "the no-break space" },
    { codePoint: 0x09, isSeparator: false, name: "the tab" },
    { codePoint: 0x20_09, isSeparator: false, name: "the thin space" },
    { codePoint: 0x2d, isSeparator: false, name: "the hyphen" },
    { codePoint: 72, isSeparator: false, name: "H" },
  ])("tells $name: $isSeparator", ({ codePoint, isSeparator }) => {
    expect(isWordSeparator(codePoint)).toBe(isSeparator);
  });
});
