import { describe, expect, it } from "vitest";

import { ARIAL_UNKERNED } from "../../../tests";

import { lineMeasureWidth } from "./line-measure-width";
import { measuredLine } from "./measured-line";

describe("lineMeasureWidth (DERIV-text-width steps 1 to 3)", () => {
  it("brings the advance to the size, then adds letter and word spacing", () => {
    // "Hello World" 10 584 units at size 16: 82.6875; 11 units, 10 letter spacings of 2; one
    // space, a word spacing of −3: 82.6875 + 20 − 3 = 99.6875.
    const measure = measuredLine(ARIAL_UNKERNED, "Hello World");

    expect(
      lineMeasureWidth(ARIAL_UNKERNED, measure, { letterSpacing: 2, size: 16, wordSpacing: -3 }),
    ).toBe(99.6875);
  });

  it("gives 0 for an empty line, whatever its spacing", () => {
    const measure = measuredLine(ARIAL_UNKERNED, "");

    expect(
      lineMeasureWidth(ARIAL_UNKERNED, measure, { letterSpacing: 5, size: 16, wordSpacing: 5 }),
    ).toBe(0);
  });
});
