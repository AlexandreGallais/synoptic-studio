// @vitest-environment happy-dom

import { describe, expect, it } from "vitest";

import { createPathElement } from "./create-path-element";
import { SVG_NAMESPACE } from "./svg-namespace";

describe("createPathElement", () => {
  it("creates a <path> element in the SVG namespace drawing the given path data", () => {
    const path = createPathElement(document, "M0 0 L120 0 L120 80 L0 80 Z");

    expect(path.namespaceURI).toBe(SVG_NAMESPACE);
    expect(path.tagName).toBe("path");
    expect(path.getAttribute("d")).toBe("M0 0 L120 0 L120 80 L0 80 Z");
  });
});
