// @vitest-environment happy-dom

import { describe, expect, it } from "vitest";

import { createSvgElement } from "./create-svg-element";
import { SVG_NAMESPACE } from "./svg-namespace";

describe("createSvgElement", () => {
  it("creates an <svg> element in the SVG namespace", () => {
    const svg = createSvgElement(document, { height: 80, width: 120, x: 0, y: 0 });

    expect(svg.namespaceURI).toBe(SVG_NAMESPACE);
    expect(svg.tagName).toBe("svg");
  });

  it("writes the view box as `x y width height`, numbers at fixed precision", () => {
    // Margin of 10 around a 120 × 80 rectangle; a derived 1 / 3 is written 0.33333 (EN-001).
    expect(
      createSvgElement(document, { height: 100, width: 140, x: -10, y: 1 / 3 }).getAttribute(
        "viewBox",
      ),
    ).toBe("-10 0.33333 140 100");
  });
});
