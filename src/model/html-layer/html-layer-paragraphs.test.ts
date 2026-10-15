import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";

import { htmlLayerParagraphs } from "./html-layer-paragraphs";

describe("htmlLayerParagraphs (REF-CSS-INLINE-3, §2.3)", () => {
  it.each([
    { content: "Hello\nWorld", paragraphs: ["Hello", "World"] },
    // A last line left empty holds a zero-width space, so that the browser lays it out.
    { content: "Hello\n", paragraphs: ["Hello", "\u{200B}"] },
    // An empty line before another ends with its line feed: kept empty.
    { content: "a\r\n\rb", paragraphs: ["a", "", "b"] },
    { content: "\n", paragraphs: ["", "\u{200B}"] },
    { content: "", paragraphs: [] },
  ])("cuts $content into $paragraphs", ({ content, paragraphs }) => {
    expect(htmlLayerParagraphs({ ...HELLO_TEXT, content })).toEqual(paragraphs);
  });

  it("writes each paragraph in the text's case", () => {
    expect(
      htmlLayerParagraphs({ ...HELLO_TEXT, content: "pump on\nvalve", textCase: "upper" }),
    ).toEqual(["PUMP ON", "VALVE"]);
  });
});
