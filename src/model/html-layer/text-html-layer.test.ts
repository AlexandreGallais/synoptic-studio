import { describe, expect, it } from "vitest";

import { ARIAL_HTML_LAYER, ARIAL_UNKERNED, HELLO_TEXT } from "../../../tests";
import { textDrawing } from "../text-drawing";

import { textHtmlLayer } from "./text-html-layer";

import type { Text } from "../text";

/** "Hello" over "World" in Arial 16 without kerning, at (0, 0). */
const HELLO_WORLD: Text = { ...HELLO_TEXT, content: "Hello\nWorld", font: "Arial" };

describe("textHtmlLayer (ADR-0053, DERIV-html-layer)", () => {
  it("[F15.AC3] gathers the frame, the library's line height, no limit and the paragraphs", () => {
    expect(
      textHtmlLayer(ARIAL_UNKERNED, HELLO_WORLD, textDrawing(ARIAL_UNKERNED, HELLO_WORLD)),
    ).toEqual(ARIAL_HTML_LAYER);
  });

  it("[F15.AC3] writes the paragraphs in the text's case, its empty lines kept, unwrapped", () => {
    const text = {
      ...HELLO_WORLD,
      boxHeight: 100,
      boxWidth: 20,
      content: "pump on\r\n\rvalve",
      isInBox: true,
      textCase: "upper",
    } as const;
    const layer = textHtmlLayer(ARIAL_UNKERNED, text, textDrawing(ARIAL_UNKERNED, text));

    // The box wraps "PUMP ON" in the library; the layer leaves the wrap to the browser.
    expect(layer.paragraphs).toEqual(["PUMP ON", "", "VALVE"]);
    expect(layer.lineLimit).toBe(5);
  });
});
