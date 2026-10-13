import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { alignmentFactor } from "./alignment-factor";
import { ALIGNMENTS } from "./alignments";

describe("alignmentFactor (DERIV-regular-polygon-fit step 5)", () => {
  it("puts no room before the shape at min, half at mid, all at max", () => {
    // SVG 2 §8.2: xMin adds nothing, xMid adds (e-width − vb-width × scale) / 2, xMax all of it.
    expect(alignmentFactor("min")).toBe(0);
    expect(alignmentFactor("mid")).toBe(0.5);
    expect(alignmentFactor("max")).toBe(1);
  });
});

/** The alignment at the other end of the axis: `min` and `max` swap, `mid` stays. */
const MIRRORED = { max: "min", mid: "mid", min: "max" } as const;

describe("alignmentFactor (properties)", () => {
  test.prop({ alignment: fc.constantFrom(...ALIGNMENTS) })(
    "gives the mirrored alignment the rest of the room",
    ({ alignment }) => {
      // Reading the axis backwards puts after the shape what was before it: f(a) + f(ā) = 1.
      expect(alignmentFactor(alignment) + alignmentFactor(MIRRORED[alignment])).toBe(1);
    },
  );
});
