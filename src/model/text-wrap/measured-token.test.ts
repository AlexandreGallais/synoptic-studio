import { describe, expect, it } from "vitest";

import { ARIAL_BOX_METRICS, HELLO_TEXT } from "../../../tests";
import { lineMeasureWidth } from "../text-block";

import { measuredToken } from "./measured-token";

describe("measuredToken (DERIV-text-box step 2)", () => {
  it("measures a token with its trailing spaces and without them", () => {
    // "Hello  ": 4667 + 2 × 569 = 5805 units, 45.35156; "Hello" 4667 units, 36.46094.
    const { visible, whole } = measuredToken(ARIAL_BOX_METRICS, "Hello  ");

    expect([whole.text, visible.text]).toEqual(["Hello  ", "Hello"]);
    expect(lineMeasureWidth(ARIAL_BOX_METRICS, whole, HELLO_TEXT)).toBe(45.3515625);
    expect(lineMeasureWidth(ARIAL_BOX_METRICS, visible, HELLO_TEXT)).toBe(36.4609375);
  });

  it("keeps a no-break space at the end, which is no space U+0020", () => {
    expect(measuredToken(ARIAL_BOX_METRICS, "Hello\u{A0}").visible.text).toBe("Hello\u{A0}");
  });
});
