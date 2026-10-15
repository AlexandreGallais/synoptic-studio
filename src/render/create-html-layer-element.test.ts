// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import { ARIAL_HTML_LAYER, HELLO_TEXT } from "../../tests";

import { createHtmlLayerElement } from "./create-html-layer-element";
import { SVG_NAMESPACE } from "./svg-namespace";
import { XHTML_NAMESPACE } from "./xhtml-namespace";

import type { Text } from "../model";

/** "Hello" over "World" in Arial 16, at (0, 0). */
const HELLO_WORLD: Text = { ...HELLO_TEXT, content: "Hello\nWorld", font: "Arial" };

describe("createHtmlLayerElement (ADR-0053, EN-041)", () => {
  const foreign = createHtmlLayerElement(document, HELLO_WORLD, ARIAL_HTML_LAYER);
  const [outer] = foreign.children;
  const content = outer?.firstElementChild;

  it("[F15.AC3] writes a foreignObject in the SVG namespace, requiring the XHTML extension", () => {
    expect([foreign.namespaceURI, foreign.tagName]).toEqual([SVG_NAMESPACE, "foreignObject"]);
    expect(foreign.getAttribute("requiredExtensions")).toBe("http://www.w3.org/1999/xhtml");
  });

  it("[F15.AC3] lets lines past its frame show, as the library draws them", () => {
    expect(foreign.getAttribute("overflow")).toBe("visible");
  });

  it("[F15.AC3] places it at the layer's frame, numbers at 5 decimals", () => {
    // −0.26171875 → −0.26172; 36.796875 → 36.79688 (Q10).
    expect(["x", "y", "width", "height"].map((name) => foreign.getAttribute(name))).toEqual([
      "0",
      "-0.26172",
      "41.78125",
      "36.79688",
    ]);
  });

  it("[F15.AC3] holds a frame div holding the content's div, both XHTML", () => {
    expect([...foreign.querySelectorAll("*")].map((element) => element.namespaceURI)).toEqual(
      Array.from({ length: 4 }, () => XHTML_NAMESPACE),
    );

    expect(outer?.getAttribute("style")).toBe("width: 41.78125px; height: 36.79688px");
    expect(content?.getAttribute("style")).toContain("line-height: 18.39844px");
    expect(content?.getAttribute("style")).toContain("white-space: pre");
  });

  it("[F15.AC3] writes one span per paragraph, a line feed between, without style", () => {
    expect([...(content?.childNodes ?? [])].map((node) => node.textContent)).toEqual([
      "Hello",
      "\n",
      "World",
    ]);

    expect(content?.querySelector("span")?.hasAttribute("style")).toBe(false);
  });

  it("[F15.AC3] escapes the content's characters when serialized", () => {
    const layer = { ...ARIAL_HTML_LAYER, paragraphs: ["a < b & c"] };
    const serializer = new XMLSerializer();
    const written = serializer.serializeToString(
      createHtmlLayerElement(document, HELLO_WORLD, layer),
    );

    expect(written).toContain("a &lt; b &amp; c");
  });

  it("[F15.AC3] writes markup typed in the content as text, one text node, escaped", () => {
    const layer = { ...ARIAL_HTML_LAYER, paragraphs: ["<b>x</b>&"] };
    const span = createHtmlLayerElement(document, HELLO_WORLD, layer).querySelector("span");
    const serializer = new XMLSerializer();

    expect([...(span?.childNodes ?? [])].map((node) => node.nodeType)).toEqual([Node.TEXT_NODE]);
    expect(span?.querySelector("b")).toBeNull();
    expect(serializer.serializeToString(span ?? document)).toContain("&lt;b&gt;x&lt;/b&gt;&amp;");
  });

  it("[F15.AC3] paints each paragraph's background on its span", () => {
    const highlighted = {
      ...HELLO_WORLD,
      backgroundColor: "#22c55e",
      backgroundOpacity: 40,
      hasBackground: true,
    };
    const spans = createHtmlLayerElement(document, highlighted, ARIAL_HTML_LAYER).querySelectorAll(
      "span",
    );

    expect([...spans].map((span) => span.getAttribute("style"))).toEqual([
      "background-color: #22c55e66",
      "background-color: #22c55e66",
    ]);
  });

  it("[F16.AC4] strikes the content's div and underlines each span when their colors differ", () => {
    const decorated = {
      ...HELLO_WORLD,
      hasUnderlineColor: true,
      isStruckThrough: true,
      isUnderlined: true,
      underlineColor: "#ff0000",
      underlineOpacity: 50,
    };
    const layer = createHtmlLayerElement(document, decorated, ARIAL_HTML_LAYER);
    const style = layer.firstElementChild?.firstElementChild?.getAttribute("style") ?? "";

    // 50 %: alpha 128, written 80; the strike-through follows the text's #000000 at 100 %.
    expect(style).toContain(
      "text-decoration-line: line-through; text-decoration-thickness: from-font; text-decoration-color: #000000ff",
    );

    expect([...layer.querySelectorAll("span")].map((span) => span.getAttribute("style"))).toEqual(
      Array.from(
        { length: 2 },
        () =>
          "text-decoration-line: underline; text-decoration-thickness: from-font; text-decoration-color: #ff000080",
      ),
    );
  });

  it("[F16.AC4] puts both strokes on the content's div while they follow the text, so that \u{2026} carries both", () => {
    const decorated = { ...HELLO_WORLD, isStruckThrough: true, isUnderlined: true };
    const layer = createHtmlLayerElement(document, decorated, ARIAL_HTML_LAYER);

    expect(layer.firstElementChild?.firstElementChild?.getAttribute("style")).toContain(
      "text-decoration-line: underline line-through; text-decoration-thickness: from-font; text-decoration-color: #000000ff",
    );

    expect([...layer.querySelectorAll("span")].map((span) => span.hasAttribute("style"))).toEqual([
      false,
      false,
    ]);
  });

  it("writes no span for an empty text", () => {
    const empty = createHtmlLayerElement(document, HELLO_WORLD, {
      ...ARIAL_HTML_LAYER,
      paragraphs: [],
    });

    expect(empty.querySelectorAll("span")).toHaveLength(0);
  });
});
