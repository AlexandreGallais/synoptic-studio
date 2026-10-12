import { describe, expect, it } from "vitest";

import { boxAspect } from "./box-aspect";

describe("boxAspect (DERIV-stretched-polygon step 4)", () => {
  it("divides the height of the bounding box by its width", () => {
    expect(
      boxAspect([
        { x: -1, y: 3 },
        { x: 3, y: 1 },
        { x: 1, y: 0 },
      ]),
    ).toBe(0.75);
  });
});
