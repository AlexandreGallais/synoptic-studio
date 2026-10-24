import { describe, expect, it } from "vitest";

import { sizeWithPanelValue } from "./size-with-panel-value";

describe("sizeWithPanelValue (F38)", () => {
  it("sets the width of 100 × 50 to 460, its height kept", () => {
    expect(sizeWithPanelValue({ height: 50, width: 100 }, "width", 460)).toEqual({
      height: 50,
      width: 460,
    });
  });

  it("sets the height of 100 × 50 to 0, its width kept", () => {
    expect(sizeWithPanelValue({ height: 50, width: 100 }, "height", 0)).toEqual({
      height: 0,
      width: 100,
    });
  });
});
