import { describe, expect, it } from "vitest";

import { ARIAL_HTML_LAYER } from "../../../tests";

import { isHtmlLayerDrawn } from "./is-html-layer-drawn";

describe("isHtmlLayerDrawn (REF-SVG2-EMBEDDED, §12.2)", () => {
  it.each([
    { isDrawn: true, height: 40, width: 50 },
    { isDrawn: false, height: 0, width: 80 },
    { isDrawn: false, height: 40, width: 0 },
    { isDrawn: false, height: 0, width: 0 },
  ])("draws a layer $width × $height: $isDrawn", ({ height, isDrawn, width }) => {
    const frame = { ...ARIAL_HTML_LAYER.frame, height, width };

    expect(isHtmlLayerDrawn({ ...ARIAL_HTML_LAYER, frame })).toBe(isDrawn);
  });
});
