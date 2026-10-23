import { describe, expect, it } from "vitest";

import { keptRectangleAt } from "../../../tests";

import { shapeBytes } from "./shape-bytes";

describe("shapeBytes (F40)", () => {
  it.each([
    // keptRectangleAt's TSDoc: 614 characters at x 10, × 2 = 1 228; 615 at x 200, 1 230.
    { bytes: 1228, shape: keptRectangleAt(10), title: "A, at x 10" },
    { bytes: 1230, shape: keptRectangleAt(200), title: "B, at x 200" },
    // `[]`: 2 characters, 4 bytes.
    { bytes: 4, shape: [], title: "an empty list" },
  ])("[F40.AC4] counts $title: $bytes bytes", ({ bytes, shape }) => {
    expect(shapeBytes(shape)).toBe(bytes);
  });
});
