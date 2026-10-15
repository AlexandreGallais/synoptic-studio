import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { subPathsToPathData } from "./sub-paths-to-path-data";

describe("subPathsToPathData (REF-SVG2-PATHS §9.3.3)", () => {
  it("[F14.AC4] writes two closed rectangles as one path data, one sub-path each", () => {
    expect(
      subPathsToPathData([
        "M0 0 L36.46094 0 L36.46094 17.875 L0 17.875 Z",
        "M0 18.39844 L41.78125 18.39844 L41.78125 36.27344 L0 36.27344 Z",
      ]),
    ).toBe(
      "M0 0 L36.46094 0 L36.46094 17.875 L0 17.875 Z M0 18.39844 L41.78125 18.39844 L41.78125 36.27344 L0 36.27344 Z",
    );
  });

  it("writes no sub-path as an empty path data", () => {
    expect(subPathsToPathData([])).toBe("");
  });

  test.prop({ count: fc.integer({ max: 20, min: 1 }) })(
    "keeps one moveto per sub-path",
    ({ count }) => {
      const data = subPathsToPathData(Array.from({ length: count }, () => "M0 0 L1 0 L1 1 Z"));

      expect(data.match(/M/gu)).toHaveLength(count);
    },
  );
});
