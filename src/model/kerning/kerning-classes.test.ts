import { describe, expect, it } from "vitest";

import { kerningClasses } from "./kerning-classes";

describe("kerningClasses (REF-OT-GPOS)", () => {
  it("gives equal rows one class, numbered from 1 as they first appear, and zero rows class 0", () => {
    expect(
      kerningClasses([
        [0, 0],
        [-1, 2],
        [0, 0],
        [3, 0],
        [-1, 2],
      ]),
    ).toEqual([0, 1, 0, 2, 1]);
  });

  it("tells rows apart by every value, not by their sum", () => {
    // 1 + 2 = 2 + 1, the rows still differ.
    expect(
      kerningClasses([
        [1, 2],
        [2, 1],
      ]),
    ).toEqual([1, 2]);
  });

  it("gives no class to no row", () => {
    expect(kerningClasses([])).toEqual([]);
  });
});
