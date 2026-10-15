import { describe, expect, it } from "vitest";

import { ARIAL_BOX_METRICS, HELLO_TEXT } from "../../../tests";
import { measuredLine } from "../text-block";

import { isMeasureFitting } from "./is-measure-fitting";

describe("isMeasureFitting (DERIV-text-box step 2)", () => {
  it("fits a measured line exactly as wide as the box, and no wider", () => {
    // At size 2048 one font unit is one user unit: "Hello" is 4667 wide.
    const hello = measuredLine(ARIAL_BOX_METRICS, "Hello");
    const text = { ...HELLO_TEXT, isInBox: true, size: 2048 };

    expect(isMeasureFitting(ARIAL_BOX_METRICS, hello, { ...text, boxWidth: 4667 })).toBe(true);
    expect(isMeasureFitting(ARIAL_BOX_METRICS, hello, { ...text, boxWidth: 4666 })).toBe(false);
  });

  it("fits a line as wide as the box within a rounding error", () => {
    // One font unit of a 10-unit em at size 1 is 0.1, plus one letter spacing of 0.2 between the
    // two units: 0.1 + 0.2 = 0.30000000000000004 in doubles, 0.3 exactly; 0.31 does not fit 0.3.
    const metrics = { ...ARIAL_BOX_METRICS, unitsPerEm: 10 };
    const measure = {
      advance: 1,
      first: 72,
      last: 101,
      separatorCount: 0,
      text: "He",
      unitCount: 2,
    };
    const text = { ...HELLO_TEXT, boxWidth: 0.3, isInBox: true, letterSpacing: 0.2, size: 1 };

    expect(isMeasureFitting(metrics, measure, text)).toBe(true);
    expect(isMeasureFitting(metrics, measure, { ...text, letterSpacing: 0.21 })).toBe(false);
  });
});
