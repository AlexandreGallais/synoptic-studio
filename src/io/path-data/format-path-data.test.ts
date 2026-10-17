import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { formatPathData } from "./format-path-data";

describe("formatPathData (REF-SVG2-PATHS §9.3.3, §9.3.9)", () => {
  it("[F14.AC4] writes two closed rectangles as one path data, one sub-path each", () => {
    expect(
      formatPathData([
        "M0 0 L36.46094 0 L36.46094 17.875 L0 17.875 Z",
        "M0 18.39844 L41.78125 18.39844 L41.78125 36.27344 L0 36.27344 Z",
      ]),
    ).toBe(
      "M0 0 L36.46094 0 L36.46094 17.875 L0 17.875 Z M0 18.39844 L41.78125 18.39844 L41.78125 36.27344 L0 36.27344 Z",
    );
  });

  it("writes commands one white space apart", () => {
    expect(formatPathData(["M0 0", "L1 0", "Z"])).toBe("M0 0 L1 0 Z");
  });

  it("writes no sub-path as an empty path data", () => {
    expect(formatPathData([])).toBe("");
  });

  test.prop({ count: fc.integer({ max: 20, min: 1 }) })(
    "keeps one moveto per sub-path",
    ({ count }) => {
      const data = formatPathData(Array.from({ length: count }, () => "M0 0 L1 0 L1 1 Z"));

      expect(data.match(/M/gu)).toHaveLength(count);
    },
  );
});
