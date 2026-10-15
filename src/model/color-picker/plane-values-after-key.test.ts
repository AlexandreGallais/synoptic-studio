import { describe, expect, it } from "vitest";

import { planeValuesAfterKey } from "./plane-values-after-key";

/** A plane at 66.7 % saturation, 60 % brightness, `#336699` at the hue 210°. */
const VALUES = { level: 60, saturation: 66.7 };

describe("planeValuesAfterKey (REF-WAI-APG-SLIDER)", () => {
  it.each([
    // Up and Down: the brightness by 1 %.
    { key: "ArrowUp", level: 61, saturation: 66.7 },
    { key: "ArrowDown", level: 59, saturation: 66.7 },
    // Left and Right, Page Up and Down, Home and End: the saturation, 66.7 ± 1, ± 10, 0, 100.
    { key: "ArrowRight", level: 60, saturation: 67.7 },
    { key: "ArrowLeft", level: 60, saturation: 65.7 },
    { key: "PageUp", level: 60, saturation: 76.7 },
    { key: "PageDown", level: 60, saturation: 56.7 },
    { key: "Home", level: 60, saturation: 0 },
    { key: "End", level: 60, saturation: 100 },
  ])("[F16.AC2] gives $saturation %, $level % on $key", ({ key, level, saturation }) => {
    const after = planeValuesAfterKey(key, VALUES);

    expect(after?.saturation).toBeCloseTo(saturation, 12);
    expect(after?.level).toBeCloseTo(level, 12);
  });

  it("keeps the brightness within 0 and 100 %", () => {
    expect(planeValuesAfterKey("ArrowUp", { level: 100, saturation: 0 })).toEqual({
      level: 100,
      saturation: 0,
    });
  });

  it("does not answer a key no slider answers", () => {
    expect(planeValuesAfterKey("Enter", VALUES)).toBeUndefined();
  });
});
