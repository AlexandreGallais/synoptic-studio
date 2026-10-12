// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import { svgNodes } from "./draw-diagrams";

describe("drawn diagrams", () => {
  it("reads Mermaid's SVG as HTML, so that a <br> in a label is accepted", () => {
    const nodes = svgNodes("<svg><foreignObject><div>a<br>b</div></foreignObject></svg>");

    expect(nodes).toHaveLength(1);
    expect(nodes[0]?.nodeName.toLowerCase()).toBe("svg");
    expect(nodes[0]?.textContent).toBe("ab");
  });
});
