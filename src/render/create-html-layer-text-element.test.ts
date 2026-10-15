// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import { ARIAL_UNKERNED, HELLO_TEXT } from "../../tests";
import { textDrawing, textHtmlLayer } from "../model";

import { createHtmlLayerTextElement } from "./create-html-layer-text-element";
import { createTextElement } from "./create-text-element";

import type { Text, TextWithHtmlLayer } from "../model";

/** "Hello" over "World" in Arial 16 without kerning, at (0, 0). */
const HELLO_WORLD: Text = { ...HELLO_TEXT, content: "Hello\nWorld", font: "Arial" };

/**
 * A text drawn in Arial without kerning, with its HTML layer.
 *
 * @param text - its content, options and angle
 * @returns the text, its drawing and its layer
 */
function withLayer(text: Text): TextWithHtmlLayer {
  const drawn = textDrawing(ARIAL_UNKERNED, text);

  return { drawn, layer: textHtmlLayer(ARIAL_UNKERNED, text, drawn), text };
}

describe("createHtmlLayerTextElement (ADR-0053, decision 1)", () => {
  const outer = createHtmlLayerTextElement(document, withLayer(HELLO_WORLD));
  const choice = outer.firstElementChild;

  it("[F15.AC3] holds one switch: the HTML layer first, the text's group second", () => {
    expect(outer.tagName).toBe("g");
    expect([...outer.children].map((child) => child.tagName)).toEqual(["switch"]);

    expect([...(choice?.children ?? [])].map((child) => child.tagName)).toEqual([
      "foreignObject",
      "g",
    ]);

    expect(choice?.firstElementChild?.getAttribute("requiredExtensions")).toBe(
      "http://www.w3.org/1999/xhtml",
    );
  });

  it("[F15.AC3] writes the text's group as F05 does, with the same content", () => {
    const alone = createTextElement(
      document,
      HELLO_WORLD,
      textDrawing(ARIAL_UNKERNED, HELLO_WORLD),
    );

    expect(choice?.lastElementChild?.outerHTML).toBe(alone.outerHTML);
  });

  it("[F15.AC3] turns the outer group by 90, and neither child", () => {
    const turned = createHtmlLayerTextElement(document, withLayer({ ...HELLO_WORLD, angle: 90 }));

    // Block 41.78125 × 36.796875 at (0, 0): middle (20.890625, 18.3984375), at 5 decimals.
    expect(turned.getAttribute("transform")).toBe("rotate(90 20.89063 18.39844)");
    expect(turned.querySelector("[transform]")).toBeNull();
  });

  it("[F15.AC3] writes no transform for a text that does not turn", () => {
    expect(outer.getAttribute("transform")).toBeNull();
  });
});
