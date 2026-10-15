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
});
