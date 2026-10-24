import { describe, expect, it } from "vitest";

import { readBoxBandPathData } from "./read-box-band-path-data";

/** A band reaching 2 outside a box's sides and 1.5 inside them, in user units. */
const REACH = { inward: 1.5, outward: 2 };

describe("readBoxBandPathData (F37, F43, AUD-045)", () => {
  it("[F43.AC2] leaves a hole in a box wider and higher than twice the inner reach", () => {
    // 4 > 2 × 1.5 = 3: the outer sides at −2 and 6, the inner at 1.5 and 2.5, the other way.
    expect(readBoxBandPathData({ maxX: 4, maxY: 4, minX: 0, minY: 0 }, REACH)).toBe(
      "M-2 -2H6V6H-2Z M1.5 1.5V2.5H2.5V1.5Z",
    );
  });

  it.each([
    // 3 is not past 2 × 1.5: the inner sides would meet; 2.5 under it.
    { width: 3, height: 10 },
    { width: 10, height: 2.5 },
  ])("[F43.AC2] fills a box $width × $height whole, no hole", ({ width, height }) => {
    const box = { maxX: width, maxY: height, minX: 0, minY: 0 };

    expect(readBoxBandPathData(box, REACH)).toBe(`M-2 -2H${width + 2}V${height + 2}H-2Z`);
  });
});
