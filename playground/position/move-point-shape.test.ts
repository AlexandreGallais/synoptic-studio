// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { openPage, readShapeModel, readText, readValue, typeAndEnter } from "../user-actions";

describe("moving a shape 0 × 0 alone", () => {
  beforeEach(() => {
    // The page as it opens: a 120 × 80 rectangle (F01), made sharp, put at (10, 20), then
    // 0 × 0: its contour has no piece, its four corners lie on (10, 20).
    openPage("");
    typeAndEnter("radius", "0");
    typeAndEnter("shape-x", "10");
    typeAndEnter("shape-y", "20");
    typeAndEnter("width", "0");
    typeAndEnter("height", "0");
  });

  it("[F04.AC3] shows X 10, Y 20, the point it lies on", () => {
    expect(readValue("shape-x")).toBe("10");
    expect(readValue("shape-y")).toBe("20");
    expect(readText("#status")).toBe("");
  });

  it("[F04.AC3] puts it at x 50 when 50 is typed in X, y kept", () => {
    // The shift is 50 − 10 = 40, from the point it lies on.
    typeAndEnter("shape-x", "50");

    expect(readShapeModel()).toMatchObject({ height: 0, width: 0, x: 50, y: 20 });
    expect(readText("#status")).toBe("");
  });
});
