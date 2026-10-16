import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_METRICS } from "../../../tests";
import { DEFAULT_FONT } from "../font";

import { lineHeight } from "./line-height";

describe("lineHeight (REF-OT-HHEA, DERIV-text-width step 4)", () => {
  it("[F05.AC3] gives Arial 16 a line of 18.39844", () => {
    // (1854 + 434 + 67) × 16 / 2048 = 2355 / 128 = 18.3984375.
    expect(lineHeight(ARIAL_METRICS, 16)).toBe(18.3984375);
  });

  it("[F05.AC3] gives Inter Regular 16 a line of 19.359375", () => {
    const regular = DEFAULT_FONT.faces.find((face) => face.weight === 400 && !face.isItalic);

    // (1984 + 494 + 0) × 16 / 2048 = 2478 / 128 = 19.359375.
    expect(lineHeight(regular?.metrics ?? ARIAL_METRICS, 16)).toBe(19.359375);
  });

  test.prop({ size: fc.integer({ max: 100_000, min: 1 }) })(
    "is the size times the em's share of the line",
    ({ size }) => {
      // 2355 font units of a 2048 em.
      expect(lineHeight(ARIAL_METRICS, size)).toBeCloseTo((2355 * size) / 2048, 6);
    },
  );
});
