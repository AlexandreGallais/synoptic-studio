import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { HUE_SLIDER_RANGE } from "./hue-slider-range";
import { PERCENT_SLIDER_RANGE } from "./percent-slider-range";
import { valueAfterSliderKey } from "./value-after-slider-key";

describe("valueAfterSliderKey (REF-WAI-APG-SLIDER)", () => {
  it.each([
    // 30.1 + 1 = 31.1; 31.1 + 10 = 41.1; the ends, 0 and 359.9.
    { expected: 31.1, key: "ArrowRight", value: 30.1 },
    { expected: 31.1, key: "ArrowUp", value: 30.1 },
    { expected: 29.1, key: "ArrowLeft", value: 30.1 },
    { expected: 29.1, key: "ArrowDown", value: 30.1 },
    { expected: 41.1, key: "PageUp", value: 31.1 },
    { expected: 21.1, key: "PageDown", value: 31.1 },
    { expected: 0, key: "Home", value: 41.1 },
    { expected: 359.9, key: "End", value: 0 },
    // Kept within the range: 359.9 + 1 stays 359.9, 5 − 10 stays 0.
    { expected: 359.9, key: "ArrowRight", value: 359.9 },
    { expected: 0, key: "PageDown", value: 5 },
  ])("[F16.AC2] takes the hue $value° to $expected° on $key", ({ expected, key, value }) => {
    expect(valueAfterSliderKey(key, value, HUE_SLIDER_RANGE)).toBeCloseTo(expected, 12);
  });

  it("[F16.AC2] takes an opacity from 0 to 100 %, End at 100", () => {
    expect(valueAfterSliderKey("End", 40, PERCENT_SLIDER_RANGE)).toBe(100);
    expect(valueAfterSliderKey("PageUp", 95, PERCENT_SLIDER_RANGE)).toBe(100);
  });

  it.each(["a", "Enter", "Tab", "toString", "constructor"])("does not answer %s", (key) => {
    expect(valueAfterSliderKey(key, 30, PERCENT_SLIDER_RANGE)).toBeUndefined();
  });

  test.prop({
    key: fc.constantFrom("ArrowDown", "ArrowLeft", "ArrowRight", "ArrowUp", "PageDown", "PageUp"),
    value: fc.integer({ max: 100, min: 0 }),
  })("stays within the range", ({ key, value }) => {
    const after = valueAfterSliderKey(key, value, PERCENT_SLIDER_RANGE) ?? NaN;

    expect(after).toBeGreaterThanOrEqual(0);
    expect(after).toBeLessThanOrEqual(100);
  });

  test.prop({ value: fc.integer({ max: 90, min: 10 }) })(
    "comes back after Right and Left, Page Up and Page Down, away from the ends",
    ({ value }) => {
      const right = valueAfterSliderKey("ArrowRight", value, PERCENT_SLIDER_RANGE) ?? NaN;
      const up = valueAfterSliderKey("PageUp", value, PERCENT_SLIDER_RANGE) ?? NaN;

      expect(valueAfterSliderKey("ArrowLeft", right, PERCENT_SLIDER_RANGE)).toBe(value);
      expect(valueAfterSliderKey("PageDown", up, PERCENT_SLIDER_RANGE)).toBe(value);
    },
  );
});
