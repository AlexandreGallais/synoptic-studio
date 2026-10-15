import { describe, expect, it } from "vitest";

import { ARIAL_HTML_LAYER, HELLO_TEXT } from "../../../tests";

import { htmlLayerToFrameStyle } from "./html-layer-to-frame-style";

/** The box 50 × 40 at (0, 0), raised by 0.26171875 (`DERIV-html-layer`, checks). */
const BOX_LAYER = { ...ARIAL_HTML_LAYER, frame: { height: 40, width: 50, x: 0, y: -0.26171875 } };

describe("htmlLayerToFrameStyle (ADR-0053, EN-041)", () => {
  it("[F15.AC3] sizes the frame of a text that fits its content as its block", () => {
    // 41.78125 × 36.796875, at 5 decimals 36.79688.
    expect(htmlLayerToFrameStyle(HELLO_TEXT, ARIAL_HTML_LAYER)).toEqual([
      ["width", "41.78125px"],
      ["height", "36.79688px"],
    ]);
  });

  it.each([
    { packing: "flex-start", verticalAlignment: "top" },
    { packing: "center", verticalAlignment: "middle" },
    { packing: "flex-end", verticalAlignment: "bottom" },
  ] as const)(
    "[F15.AC3] places the block down a box $verticalAlignment as a flex column packed $packing",
    ({ packing, verticalAlignment }) => {
      const text = { ...HELLO_TEXT, isInBox: true, verticalAlignment };

      expect(htmlLayerToFrameStyle(text, BOX_LAYER)).toEqual([
        ["width", "50px"],
        ["height", "40px"],
        ["display", "flex"],
        ["flex-direction", "column"],
        ["justify-content", packing],
      ]);
    },
  );
});
