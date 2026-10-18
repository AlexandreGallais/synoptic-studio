import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { HUE_SLIDER_RANGE } from "./hue-slider-range";
import { PERCENT_SLIDER_RANGE } from "./percent-slider-range";
import { valueAfterSliderKey } from "./value-after-slider-key";

describe("valueAfterSliderKey (REF-WAI-APG-SLIDER)", () => {
  it.each([
    // 30 + 1 = 31; 31 + 10 = 41; the ends, 0 and 359 (ADR-0058, decision 4).
    { expected: 31, key: "ArrowRight", value: 30 },
    { expected: 31, key: "ArrowUp", value: 30 },
    { expected: 29, key: "ArrowLeft", value: 30 },
    { expected: 29, key: "ArrowDown", value: 30 },
    { expected: 40, key: "PageUp", value: 30 },
    { expected: 21, key: "PageDown", value: 31 },
    { expected: 0, key: "Home", value: 41 },
    { expected: 359, key: "End", value: 0 },
    // Kept within the range: 359 + 1 stays 359, 5 − 10 stays 0.
    { expected: 359, key: "ArrowRight", value: 359 },
    { expected: 0, key: "PageDown", value: 5 },
  ])("[F16.AC2] takes the hue $value° to $expected° on $key", ({ expected, key, value }) => {
    expect(valueAfterSliderKey(key, value, HUE_SLIDER_RANGE)).toBe(expected);
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
