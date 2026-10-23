// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import { selectElement } from "./select-element";

/**
 * Parses a page of its own, as `loadPage` parses the playground's.
 *
 * @param html - its elements, written as the page would hold them
 * @returns the page parsed
 */
function documentWith(html: string): Document {
  const parser = new DOMParser();

  return parser.parseFromString(html, "text/html");
}

describe("selectElement, by the document's map of ids (EN-150, REF-HAPPY-DOM-QUERY)", () => {
  it("[F43.AC1] returns the element of the id, the first in the page's order", () => {
    const own = documentWith('<p id="a">first</p><p id="a">second</p><span id="b"></span>');

    expect(selectElement(own, "a").textContent).toBe("first");
    expect(selectElement(own, "b").tagName).toBe("SPAN");
  });

  it("[F43.AC1] reads the id as written, a dot in it not read as a class", () => {
    const own = documentWith('<p id="shape.x"></p>');

    expect(selectElement(own, "shape.x").id).toBe("shape.x");
  });

  it("[F43.AC1] throws for an id the page lacks, naming it", () => {
    const own = documentWith('<p id="a"></p>');

    expect(() => selectElement(own, "missing")).toThrow(
      "Playground page without #missing (docs/backlog/stories/E01-F02-US-007-shape-selector-playground.md).",
    );
  });
});
