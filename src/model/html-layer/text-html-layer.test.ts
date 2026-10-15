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

  it("[F15.AC3] gives a content ending with a line break as many lines as the library, bottom-aligned", () => {
    const text = {
      ...HELLO_WORLD,
      boxHeight: 100,
      boxWidth: 100,
      content: "Hello\n",
      isInBox: true,
      verticalAlignment: "bottom",
    } as const;
    const drawn = textDrawing(ARIAL_UNKERNED, text);
    const layer = textHtmlLayer(ARIAL_UNKERNED, text, drawn);

    // Two lines in both: "Hello" and an empty one, the browser's held open by U+200B; the
    // blocks, 2 · 18.3984375 tall, bottom-aligned alike, the box raised by 0.26171875.
    expect(drawn.lines.map((line) => line.content)).toEqual(["Hello", ""]);
    expect(layer.paragraphs).toEqual(["Hello", "\u{200B}"]);
    expect(layer.frame).toEqual({ height: 100, width: 100, x: 0, y: -0.26171875 });
  });
});
