import { describe, expect, it } from "vitest";

import { ARIAL_BOX_METRICS, ARIAL_UNKERNED } from "../../../tests";

import { textEllipsis } from "./text-ellipsis";

describe("textEllipsis (REF-CSS-OVERFLOW-3)", () => {
  it("[F14.AC2] is the ellipsis character when the face has its glyph", () => {
    // Arial maps U+2026 to a glyph 2048 units wide.
    expect(textEllipsis(ARIAL_BOX_METRICS)).toBe("\u{2026}");
  });

  it("[F14.AC2] is three full stops when the face lacks it", () => {
    // The fixture's unkerned Arial maps no glyph to U+2026.
    expect(textEllipsis(ARIAL_UNKERNED)).toBe("...");
  });
});
