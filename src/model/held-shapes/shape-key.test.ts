import { describe, expect, it } from "vitest";

import { shapeKey } from "./shape-key";

describe("shapeKey (EN-147)", () => {
  it("[F41.AC2] gives a shape the same key at every call", () => {
    const shape = ["a"];

    expect(shapeKey(shape)).toBe(shapeKey(shape));
  });

  it("[F41.AC2] gives two shapes of equal texts two keys, the second the next", () => {
    const first = shapeKey(["a"]);
    const second = shapeKey(["a"]);

    expect(second).toBe(first + 1);
    expect(Number.isSafeInteger(first) && first >= 0).toBe(true);
  });
});
